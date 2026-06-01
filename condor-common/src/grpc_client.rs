use crate::JobStatus;
use crate::proto::{
    CreateJobGroupRequest, CreateJobGroupResponse, CreateJobRequest, CreateJobResponse,
    GetJobGroupRequest, GetJobGroupResponse, GetJobRequest, GetJobResponse, PopJobFromGroupRequest,
    PopJobFromGroupResponse, UpdateJobStatusRequest, UpdateJobStatusResponse,
    message_service_client::MessageServiceClient,
};

pub struct CondorGrpcClient {
    client: MessageServiceClient<tonic::transport::Channel>,
}

impl CondorGrpcClient {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = MessageServiceClient::connect(addr.to_owned()).await?;
        Ok(Self { client })
    }

    pub async fn create_job_group(
        &mut self,
        name: String,
        template: String,
    ) -> Result<CreateJobGroupResponse, tonic::Status> {
        self.client
            .create_job_group(CreateJobGroupRequest { name, template })
            .await
            .map(|r| r.into_inner())
    }

    pub async fn get_job_group(
        &mut self,
        id: String,
    ) -> Result<GetJobGroupResponse, tonic::Status> {
        self.client
            .get_job_group(GetJobGroupRequest { id })
            .await
            .map(|r| r.into_inner())
    }

    pub async fn create_job(
        &mut self,
        group_id: String,
        name: String,
        parameters: std::collections::HashMap<String, String>,
    ) -> Result<CreateJobResponse, tonic::Status> {
        self.client
            .create_job(CreateJobRequest {
                group_id,
                name,
                parameters,
            })
            .await
            .map(|r| r.into_inner())
    }

    pub async fn get_job(&mut self, id: String) -> Result<GetJobResponse, tonic::Status> {
        self.client
            .get_job(GetJobRequest { id })
            .await
            .map(|r| r.into_inner())
    }

    pub async fn update_job_status(
        &mut self,
        id: String,
        status: JobStatus,
    ) -> Result<UpdateJobStatusResponse, tonic::Status> {
        self.client
            .update_job_status(UpdateJobStatusRequest {
                id,
                status: status.to_proto_value(),
            })
            .await
            .map(|r| r.into_inner())
    }

    pub async fn pop_job_from_group(
        &mut self,
        group_id: String,
    ) -> Result<PopJobFromGroupResponse, tonic::Status> {
        self.client
            .pop_job_from_group(PopJobFromGroupRequest { group_id })
            .await
            .map(|r| r.into_inner())
    }
}
