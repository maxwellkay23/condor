use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct FindClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl FindClient<'_> {
    pub async fn text(
        &self,
        directory: Option<&str>,
        pattern: &str,
        workspace: Option<&str>,
    ) -> Result<Vec<types::FindTextResponseItem>, CondorOpencodeError> {
        let response = self.client.find_text(directory, pattern, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn files(
        &self,
        directory: Option<&str>,
        dirs: Option<types::FindFilesDirs>,
        limit: Option<std::num::NonZeroU64>,
        query: &str,
        type_: Option<types::FindFilesType>,
        workspace: Option<&str>,
    ) -> Result<Vec<String>, CondorOpencodeError> {
        let response = self
            .client
            .find_files(directory, dirs, limit, query, type_, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn symbols(
        &self,
        directory: Option<&str>,
        query: &str,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Symbol>, CondorOpencodeError> {
        let response = self
            .client
            .find_symbols(directory, query, workspace)
            .await?;
        Ok(response.into_inner())
    }
}
