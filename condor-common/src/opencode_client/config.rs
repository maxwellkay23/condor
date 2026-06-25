use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct ConfigClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl ConfigClient<'_> {
    pub async fn get(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Config, CondorOpencodeError> {
        let response = self.client.config_get(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::Config,
    ) -> Result<types::Config, CondorOpencodeError> {
        let response = self
            .client
            .config_update(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn providers(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::ConfigProvidersResponse, CondorOpencodeError> {
        let response = self.client.config_providers(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
