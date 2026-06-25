use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct FileClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl FileClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        path: &str,
        workspace: Option<&str>,
    ) -> Result<Vec<types::FileNode>, CondorOpencodeError> {
        let response = self.client.file_list(directory, path, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn read(
        &self,
        directory: Option<&str>,
        path: &str,
        workspace: Option<&str>,
    ) -> Result<types::FileContent, CondorOpencodeError> {
        let response = self.client.file_read(directory, path, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::File>, CondorOpencodeError> {
        let response = self.client.file_status(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
