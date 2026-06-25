use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct FormatterClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl FormatterClient<'_> {
    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::FormatterStatus>, CondorOpencodeError> {
        let response = self.client.formatter_status(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
