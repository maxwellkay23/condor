use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct LspClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl LspClient<'_> {
    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::LspStatus>, CondorOpencodeError> {
        let response = self.client.lsp_status(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
