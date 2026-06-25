use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct CommandClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl CommandClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Command>, CondorOpencodeError> {
        let response = self.client.command_list(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
