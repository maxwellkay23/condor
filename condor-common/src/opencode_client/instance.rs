use crate::CondorOpencodeError;

pub struct InstanceClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl InstanceClient<'_> {
    pub async fn dispose(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.instance_dispose(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
