use std::collections::HashMap;

use crate::CondorError;

#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Created,
    Running,
    Failed,
    Completed,
}

impl JobStatus {
    pub fn from_str(s: &str) -> Result<Self, CondorError> {
        match s {
            "Created" => Ok(JobStatus::Created),
            "Running" => Ok(JobStatus::Running),
            "Failed" => Ok(JobStatus::Failed),
            "Completed" => Ok(JobStatus::Completed),
            other => Err(CondorError::InvalidJobStatus(format!(
                "unknown job status: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Created => "Created",
            JobStatus::Running => "Running",
            JobStatus::Failed => "Failed",
            JobStatus::Completed => "Completed",
        }
    }

    pub fn to_proto_value(&self) -> i32 {
        match self {
            JobStatus::Created => 0,
            JobStatus::Running => 1,
            JobStatus::Failed => 2,
            JobStatus::Completed => 3,
        }
    }

    pub fn from_proto_value(v: i32) -> Result<Self, CondorError> {
        match v {
            0 => Ok(JobStatus::Created),
            1 => Ok(JobStatus::Running),
            2 => Ok(JobStatus::Failed),
            3 => Ok(JobStatus::Completed),
            other => Err(CondorError::InvalidJobStatus(format!(
                "unknown proto job status: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobGroup {
    pub id: String,
    pub name: String,
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub status: JobStatus,
}
