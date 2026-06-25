use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct PathClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl PathClient<'_> {
    pub async fn get(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Path, CondorOpencodeError> {
        let response = self.client.path_get(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
