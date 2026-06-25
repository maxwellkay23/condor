use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct ProviderClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl ProviderClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::ProviderListResponse, CondorOpencodeError> {
        let response = self.client.provider_list(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn auth(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<
        std::collections::HashMap<String, Vec<types::ProviderAuthMethod>>,
        CondorOpencodeError,
    > {
        let response = self.client.provider_auth(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn oauth_authorize(
        &self,
        provider_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ProviderOauthAuthorizeBody,
    ) -> Result<types::ProviderAuthAuthorization, CondorOpencodeError> {
        let response = self
            .client
            .provider_oauth_authorize(provider_id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn oauth_callback(
        &self,
        provider_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ProviderOauthCallbackBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .provider_oauth_callback(provider_id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }
}
