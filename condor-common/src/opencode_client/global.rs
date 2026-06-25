use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct GlobalClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl GlobalClient<'_> {
    pub async fn health(&self) -> Result<types::GlobalHealthResponse, CondorOpencodeError> {
        let response = self.client.global_health().await?;
        Ok(response.into_inner())
    }

    pub async fn config_get(&self) -> Result<types::Config, CondorOpencodeError> {
        let response = self.client.global_config_get().await?;
        Ok(response.into_inner())
    }

    pub async fn config_update(
        &self,
        body: &types::Config,
    ) -> Result<types::Config, CondorOpencodeError> {
        let response = self.client.global_config_update(body).await?;
        Ok(response.into_inner())
    }

    pub async fn dispose(&self) -> Result<bool, CondorOpencodeError> {
        let response = self.client.global_dispose().await?;
        Ok(response.into_inner())
    }

    pub async fn upgrade(
        &self,
        body: &types::GlobalUpgradeBody,
    ) -> Result<types::GlobalUpgradeResponse, CondorOpencodeError> {
        let response = self.client.global_upgrade(body).await?;
        Ok(response.into_inner())
    }
}
