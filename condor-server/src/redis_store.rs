use redis::AsyncCommands;
use redis::aio::ConnectionManager;

use condor_common::{Job, JobGroup, JobStatus};

use crate::error::CondorServerError;

#[derive(Clone)]
pub struct RedisStore {
    conn: ConnectionManager,
    root: String,
}

impl RedisStore {
    pub async fn connect(url: &str, root: &str) -> Result<Self, CondorServerError> {
        let client = redis::Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self {
            conn,
            root: root.to_owned(),
        })
    }

    fn gen_key(&self, parts: &[&str]) -> String {
        let mut key = self.root.clone();
        for p in parts {
            key.push_str("::");
            key.push_str(p);
        }
        key
    }

    // ── Job Groups ──

    pub async fn create_job_group(
        &self,
        id: &str,
        name: &str,
        template: &str,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job_group", id]);
        let templates_key = self.gen_key(&["condor", "job_group", id, "templates"]);
        let set_key = self.gen_key(&["condor", "jobgroups"]);
        redis::pipe()
            .hset(&hash_key, "name", name)
            .hset(&hash_key, "template_count", 1)
            .hset(&templates_key, "1", template)
            .sadd(&set_key, id)
            .exec_async(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn get_job_group(&self, id: &str) -> Result<JobGroup, CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job_group", id]);
        let templates_key = self.gen_key(&["condor", "job_group", id, "templates"]);

        let (name, template_count): (String, i32) = redis::cmd("HMGET")
            .arg(&hash_key)
            .arg("name")
            .arg("template_count")
            .query_async(&mut conn)
            .await?;

        if name.is_empty() {
            return Err(CondorServerError::NotFound(format!(
                "job group {id} not found"
            )));
        }

        let version_str = template_count.to_string();
        let template: String = redis::cmd("HGET")
            .arg(&templates_key)
            .arg(&version_str)
            .query_async(&mut conn)
            .await?;

        Ok(JobGroup {
            id: id.to_owned(),
            name,
            template,
            current_version: template_count,
        })
    }

    pub async fn update_job_group(
        &self,
        id: &str,
        template: &str,
    ) -> Result<i32, CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job_group", id]);
        let templates_key = self.gen_key(&["condor", "job_group", id, "templates"]);

        let template_count: i32 = redis::cmd("HINCRBY")
            .arg(&hash_key)
            .arg("template_count")
            .arg(1)
            .query_async(&mut conn)
            .await?;

        let version_str = template_count.to_string();
        conn.hset::<&str, &str, &str, ()>(&templates_key, &version_str, template)
            .await?;

        Ok(template_count)
    }

    pub async fn list_job_groups(&self) -> Result<Vec<(String, String, i32)>, CondorServerError> {
        let mut conn = self.conn.clone();
        let set_key = self.gen_key(&["condor", "jobgroups"]);
        let ids: Vec<String> = redis::cmd("SMEMBERS")
            .arg(&set_key)
            .query_async(&mut conn)
            .await?;

        let mut groups = Vec::new();
        for id in &ids {
            let hash_key = self.gen_key(&["condor", "job_group", id]);
            let (name, template_count): (String, i32) = redis::cmd("HMGET")
                .arg(&hash_key)
                .arg("name")
                .arg("template_count")
                .query_async(&mut conn)
                .await?;
            if !name.is_empty() {
                groups.push((id.clone(), name, template_count));
            }
        }
        Ok(groups)
    }

    pub async fn delete_job_group(&self, id: &str) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job_group", id]);
        let templates_key = self.gen_key(&["condor", "job_group", id, "templates"]);
        let jobs_set_key = self.gen_key(&["condor", "job_group", id, "jobs"]);
        let set_key = self.gen_key(&["condor", "jobgroups"]);

        let job_ids: Vec<String> = redis::cmd("SMEMBERS")
            .arg(&jobs_set_key)
            .query_async(&mut conn)
            .await?;

        let mut pipe = redis::pipe();
        for job_id in &job_ids {
            let job_key = self.gen_key(&["condor", "job", job_id]);
            let params_key = self.gen_key(&["condor", "job", job_id, "params"]);
            pipe.del(job_key).del(params_key);
        }
        pipe.del(&hash_key)
            .del(&templates_key)
            .del(&jobs_set_key)
            .srem(&set_key, id)
            .exec_async(&mut conn)
            .await?;

        Ok(())
    }

    // ── Jobs ──

    pub async fn create_job(
        &self,
        id: &str,
        group_id: &str,
        name: &str,
        parameters: &std::collections::HashMap<String, String>,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job", id]);
        let params_key = self.gen_key(&["condor", "job", id, "params"]);
        let jobs_set_key = self.gen_key(&["condor", "job_group", group_id, "jobs"]);
        let group_hash_key = self.gen_key(&["condor", "job_group", group_id]);

        let template_count: i32 = redis::cmd("HGET")
            .arg(&group_hash_key)
            .arg("template_count")
            .query_async(&mut conn)
            .await?;

        let mut pipe = redis::pipe();
        pipe.hset(&hash_key, "name", name)
            .hset(&hash_key, "group_id", group_id)
            .hset(&hash_key, "status", "Created")
            .hset(&hash_key, "template_version", template_count);
        for (k, v) in parameters {
            pipe.hset(&params_key, k.as_str(), v.as_str());
        }
        pipe.sadd(&jobs_set_key, id).exec_async(&mut conn).await?;
        Ok(())
    }

    pub async fn get_job(&self, id: &str) -> Result<Job, CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job", id]);
        let params_key = self.gen_key(&["condor", "job", id, "params"]);

        let (name, group_id, status, template_version): (String, String, String, i32) =
            redis::cmd("HMGET")
                .arg(&hash_key)
                .arg("name")
                .arg("group_id")
                .arg("status")
                .arg("template_version")
                .query_async(&mut conn)
                .await?;

        if name.is_empty() {
            return Err(CondorServerError::NotFound(format!("job {id} not found")));
        }

        let parameters: std::collections::HashMap<String, String> = redis::cmd("HGETALL")
            .arg(&params_key)
            .query_async(&mut conn)
            .await?;

        Ok(Job {
            id: id.to_owned(),
            group_id,
            name,
            parameters,
            status: JobStatus::from_str(&status)?,
            template_version,
        })
    }

    pub async fn get_group_template(
        &self,
        group_id: &str,
        version: i32,
    ) -> Result<String, CondorServerError> {
        let mut conn = self.conn.clone();
        let templates_key = self.gen_key(&["condor", "job_group", group_id, "templates"]);
        let template: Option<String> = redis::cmd("HGET")
            .arg(&templates_key)
            .arg(version.to_string())
            .query_async(&mut conn)
            .await?;
        template.ok_or_else(|| {
            CondorServerError::NotFound(format!(
                "template version {version} for group {group_id} not found"
            ))
        })
    }

    pub async fn list_jobs(
        &self,
        group_id: &str,
    ) -> Result<Vec<(String, String, JobStatus, i32)>, CondorServerError> {
        let mut conn = self.conn.clone();
        let jobs_set_key = self.gen_key(&["condor", "job_group", group_id, "jobs"]);
        let ids: Vec<String> = redis::cmd("SMEMBERS")
            .arg(&jobs_set_key)
            .query_async(&mut conn)
            .await?;

        let mut jobs = Vec::new();
        for id in &ids {
            let hash_key = self.gen_key(&["condor", "job", id]);
            let (name, status, template_version): (String, String, i32) = redis::cmd("HMGET")
                .arg(&hash_key)
                .arg("name")
                .arg("status")
                .arg("template_version")
                .query_async(&mut conn)
                .await?;
            if !name.is_empty() {
                let status = JobStatus::from_str(&status)?;
                jobs.push((id.clone(), name, status, template_version));
            }
        }
        Ok(jobs)
    }

    pub async fn delete_job(&self, id: &str) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job", id]);
        let params_key = self.gen_key(&["condor", "job", id, "params"]);

        let group_id: Option<String> = redis::cmd("HGET")
            .arg(&hash_key)
            .arg("group_id")
            .query_async(&mut conn)
            .await?;

        if let Some(group_id) = group_id {
            let jobs_set_key = self.gen_key(&["condor", "job_group", &group_id, "jobs"]);
            redis::pipe()
                .del(&hash_key)
                .del(&params_key)
                .srem(&jobs_set_key, id)
                .exec_async(&mut conn)
                .await?;
        }
        Ok(())
    }

    pub async fn update_job_status(
        &self,
        id: &str,
        status: &JobStatus,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = self.gen_key(&["condor", "job", id]);
        conn.hset::<&str, &str, &str, ()>(&hash_key, "status", status.as_str())
            .await?;
        Ok(())
    }

    pub async fn pop_job_from_group(
        &self,
        group_id: &str,
    ) -> Result<Option<String>, CondorServerError> {
        let mut conn = self.conn.clone();
        let jobs_set_key = self.gen_key(&["condor", "job_group", group_id, "jobs"]);
        let job_id: Option<String> = redis::cmd("SPOP")
            .arg(&jobs_set_key)
            .query_async(&mut conn)
            .await?;
        Ok(job_id)
    }
}
