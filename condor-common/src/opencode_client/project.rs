use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct ProjectClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl ProjectClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Project>, CondorOpencodeError> {
        let response = self.client.project_list(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn current(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Project, CondorOpencodeError> {
        let response = self.client.project_current(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &self,
        project_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ProjectUpdateBody,
    ) -> Result<types::Project, CondorOpencodeError> {
        let response = self
            .client
            .project_update(project_id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn init_git(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Project, CondorOpencodeError> {
        let response = self.client.project_init_git(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
