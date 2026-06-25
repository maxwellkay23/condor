use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct AuthClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl AuthClient<'_> {
    pub async fn set(
        &self,
        provider_id: &str,
        body: &types::Auth,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.auth_set(provider_id, body).await?;
        Ok(response.into_inner())
    }

    pub async fn remove(&self, provider_id: &str) -> Result<bool, CondorOpencodeError> {
        let response = self.client.auth_remove(provider_id).await?;
        Ok(response.into_inner())
    }
}
