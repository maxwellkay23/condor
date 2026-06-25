use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct AppClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl AppClient<'_> {
    pub async fn log(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::AppLogBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.app_log(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn agents(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Agent>, CondorOpencodeError> {
        let response = self.client.app_agents(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn skills(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::AppSkillsResponseItem>, CondorOpencodeError> {
        let response = self.client.app_skills(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
