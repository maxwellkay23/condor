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

    pub async fn get_job_group(&self, id: &str) -> Result<JobGroup, CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = format!("{}::condor::job_group::{id}", self.root);
        let (name, template): (String, String) = redis::cmd("HMGET")
            .arg(&hash_key)
            .arg("name")
            .arg("template")
            .query_async(&mut conn)
            .await?;
        Ok(JobGroup {
            id: id.to_owned(),
            name,
            template,
        })
    }

    pub async fn create_job_group(
        &self,
        id: &str,
        name: &str,
        template: &str,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = format!("{}::condor::job_group::{id}", self.root);
        let set_key = format!("{}::condor::jobgroups", self.root);
        redis::pipe()
            .hset(&hash_key, "name", name)
            .hset(&hash_key, "template", template)
            .sadd(&set_key, id)
            .exec_async(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn create_job(
        &self,
        id: &str,
        group_id: &str,
        name: &str,
        parameters: &std::collections::HashMap<String, String>,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = format!("{}::condor::job::{id}", self.root);
        let params_key = format!("{}::condor::job::{id}::params", self.root);
        let jobs_set_key = format!("{}::condor::job_group::{group_id}::jobs", self.root);

        let mut pipe = redis::pipe();
        pipe.hset(&hash_key, "name", name)
            .hset(&hash_key, "group_id", group_id)
            .hset(&hash_key, "status", "Created");
        for (k, v) in parameters {
            pipe.hset(&params_key, k.as_str(), v.as_str());
        }
        pipe.sadd(&jobs_set_key, id).exec_async(&mut conn).await?;
        Ok(())
    }

    pub async fn get_job(&self, id: &str) -> Result<Job, CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = format!("{}::condor::job::{id}", self.root);
        let params_key = format!("{}::condor::job::{id}::params", self.root);

        let (name, group_id, status): (String, String, String) = redis::cmd("HMGET")
            .arg(&hash_key)
            .arg("name")
            .arg("group_id")
            .arg("status")
            .query_async(&mut conn)
            .await?;

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
        })
    }

    pub async fn update_job_status(
        &self,
        id: &str,
        status: &JobStatus,
    ) -> Result<(), CondorServerError> {
        let mut conn = self.conn.clone();
        let hash_key = format!("{}::condor::job::{id}", self.root);
        conn.hset::<&str, &str, &str, ()>(&hash_key, "status", status.as_str())
            .await?;
        Ok(())
    }

    pub async fn pop_job_from_group(
        &self,
        group_id: &str,
    ) -> Result<Option<String>, CondorServerError> {
        let mut conn = self.conn.clone();
        let jobs_set_key = format!("{}::condor::job_group::{group_id}::jobs", self.root);
        let job_id: Option<String> = redis::cmd("SPOP")
            .arg(&jobs_set_key)
            .query_async(&mut conn)
            .await?;
        Ok(job_id)
    }
}
