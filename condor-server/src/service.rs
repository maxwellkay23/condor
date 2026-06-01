use condor_common::proto::{
    CreateJobGroupRequest, CreateJobGroupResponse, CreateJobRequest, CreateJobResponse,
    GetJobGroupRequest, GetJobGroupResponse, GetJobRequest, GetJobResponse, PopJobFromGroupRequest,
    PopJobFromGroupResponse, UpdateJobStatusRequest, UpdateJobStatusResponse,
    message_service_server::MessageService,
};
use tonic::Request;

use crate::redis_store::RedisStore;

pub struct MessageServiceImpl {
    pub store: RedisStore,
}

#[tonic::async_trait]
impl MessageService for MessageServiceImpl {
    async fn create_job_group(
        &self,
        request: Request<CreateJobGroupRequest>,
    ) -> Result<tonic::Response<CreateJobGroupResponse>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::new_v4().to_string();

        self.store
            .create_job_group(&id, &req.name, &req.template)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!(
            "create_job_group: name={}, template={}, id={}",
            req.name, req.template, id
        );
        Ok(tonic::Response::new(CreateJobGroupResponse { id }))
    }

    async fn get_job_group(
        &self,
        request: Request<GetJobGroupRequest>,
    ) -> Result<tonic::Response<GetJobGroupResponse>, tonic::Status> {
        let id = request.into_inner().id;

        let group = self
            .store
            .get_job_group(&id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(GetJobGroupResponse {
            id: group.id,
            name: group.name,
            template: group.template,
        }))
    }

    async fn create_job(
        &self,
        request: Request<CreateJobRequest>,
    ) -> Result<tonic::Response<CreateJobResponse>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::new_v4().to_string();

        self.store
            .create_job(&id, &req.group_id, &req.name, &req.parameters)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!(
            "create_job: id={}, group_id={}, name={}",
            id, req.group_id, req.name
        );
        Ok(tonic::Response::new(CreateJobResponse { id }))
    }

    async fn get_job(
        &self,
        request: Request<GetJobRequest>,
    ) -> Result<tonic::Response<GetJobResponse>, tonic::Status> {
        let id = request.into_inner().id;

        let job = self
            .store
            .get_job(&id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(GetJobResponse {
            id: job.id,
            group_id: job.group_id,
            name: job.name,
            parameters: job.parameters,
            status: job.status.to_proto_value(),
        }))
    }

    async fn update_job_status(
        &self,
        request: Request<UpdateJobStatusRequest>,
    ) -> Result<tonic::Response<UpdateJobStatusResponse>, tonic::Status> {
        let req = request.into_inner();
        let status = condor_common::JobStatus::from_proto_value(req.status)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        self.store
            .update_job_status(&req.id, &status)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!(
            "update_job_status: id={}, status={}",
            req.id,
            status.as_str()
        );
        Ok(tonic::Response::new(UpdateJobStatusResponse {
            success: true,
        }))
    }

    async fn pop_job_from_group(
        &self,
        request: Request<PopJobFromGroupRequest>,
    ) -> Result<tonic::Response<PopJobFromGroupResponse>, tonic::Status> {
        let group_id = request.into_inner().group_id;

        let job_id = self
            .store
            .pop_job_from_group(&group_id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let job_id = job_id.unwrap_or_default();
        println!(
            "pop_job_from_group: group_id={}, job_id={}",
            group_id, job_id
        );
        Ok(tonic::Response::new(PopJobFromGroupResponse { job_id }))
    }
}
