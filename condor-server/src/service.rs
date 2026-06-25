use condor_common::proto::{
    CreateJobGroupRequest, CreateJobGroupResponse, CreateJobRequest, CreateJobResponse,
    DeleteJobGroupRequest, DeleteJobGroupResponse, DeleteJobRequest, DeleteJobResponse,
    GetGroupTemplateRequest, GetGroupTemplateResponse, GetJobGroupRequest, GetJobGroupResponse,
    GetJobRequest, GetJobResponse, ListJobGroupsRequest, ListJobGroupsResponse, ListJobsRequest,
    ListJobsResponse, PopJobFromGroupRequest, PopJobFromGroupResponse, UpdateJobGroupRequest,
    UpdateJobGroupResponse, UpdateJobStatusRequest, UpdateJobStatusResponse,
    message_service_server::MessageService,
};
use tonic::Request;

use crate::nats::NatsPublisher;
use crate::redis_store::RedisStore;

pub struct MessageServiceImpl {
    pub store: RedisStore,
    pub publisher: Option<NatsPublisher>,
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
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        Ok(tonic::Response::new(GetJobGroupResponse {
            id: group.id,
            name: group.name,
            template: group.template,
            current_version: group.current_version,
        }))
    }

    async fn get_group_template(
        &self,
        request: Request<GetGroupTemplateRequest>,
    ) -> Result<tonic::Response<GetGroupTemplateResponse>, tonic::Status> {
        let req = request.into_inner();
        let template = self
            .store
            .get_group_template(&req.group_id, req.version)
            .await
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        Ok(tonic::Response::new(GetGroupTemplateResponse { template }))
    }

    async fn update_job_group(
        &self,
        request: Request<UpdateJobGroupRequest>,
    ) -> Result<tonic::Response<UpdateJobGroupResponse>, tonic::Status> {
        let req = request.into_inner();
        let new_version = self
            .store
            .update_job_group(&req.id, &req.template)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!(
            "update_job_group: id={}, new_version={}",
            req.id, new_version
        );
        Ok(tonic::Response::new(UpdateJobGroupResponse { new_version }))
    }

    async fn list_job_groups(
        &self,
        _request: Request<ListJobGroupsRequest>,
    ) -> Result<tonic::Response<ListJobGroupsResponse>, tonic::Status> {
        let groups = self
            .store
            .list_job_groups()
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let job_groups = groups
            .into_iter()
            .map(
                |(id, name, current_version)| condor_common::proto::JobGroupSummary {
                    id,
                    name,
                    current_version,
                },
            )
            .collect();

        Ok(tonic::Response::new(ListJobGroupsResponse { job_groups }))
    }

    async fn delete_job_group(
        &self,
        request: Request<DeleteJobGroupRequest>,
    ) -> Result<tonic::Response<DeleteJobGroupResponse>, tonic::Status> {
        let id = request.into_inner().id;
        self.store
            .delete_job_group(&id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!("delete_job_group: id={}", id);
        Ok(tonic::Response::new(DeleteJobGroupResponse {}))
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
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        Ok(tonic::Response::new(GetJobResponse {
            id: job.id,
            group_id: job.group_id,
            name: job.name,
            parameters: job.parameters,
            status: job.status.to_proto_value(),
            template_version: job.template_version,
        }))
    }

    async fn list_jobs(
        &self,
        request: Request<ListJobsRequest>,
    ) -> Result<tonic::Response<ListJobsResponse>, tonic::Status> {
        let group_id = request.into_inner().group_id;
        let jobs = self
            .store
            .list_jobs(&group_id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let jobs = jobs
            .into_iter()
            .map(
                |(id, name, status, template_version)| condor_common::proto::JobSummary {
                    id,
                    name,
                    status: status.to_proto_value(),
                    template_version,
                },
            )
            .collect();

        Ok(tonic::Response::new(ListJobsResponse { jobs }))
    }

    async fn delete_job(
        &self,
        request: Request<DeleteJobRequest>,
    ) -> Result<tonic::Response<DeleteJobResponse>, tonic::Status> {
        let id = request.into_inner().id;
        self.store
            .delete_job(&id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        println!("delete_job: id={}", id);
        Ok(tonic::Response::new(DeleteJobResponse {}))
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

        if status == condor_common::JobStatus::Completed
            && !req.group_id.is_empty()
            && let Some(ref publisher) = self.publisher
        {
            publisher
                .publish_job_completed(&req.group_id, &req.id)
                .await;
        }

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
