use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct VcsClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl VcsClient<'_> {
    pub async fn get(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::VcsInfo, CondorOpencodeError> {
        let response = self.client.vcs_get(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::VcsFileStatus>, CondorOpencodeError> {
        let response = self.client.vcs_status(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn diff(
        &self,
        context: Option<u64>,
        directory: Option<&str>,
        mode: types::VcsDiffMode,
        workspace: Option<&str>,
    ) -> Result<Vec<types::VcsFileDiff>, CondorOpencodeError> {
        let response = self
            .client
            .vcs_diff(context, directory, mode, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn apply(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::VcsApplyBody,
    ) -> Result<types::VcsApplyResponse, CondorOpencodeError> {
        let response = self.client.vcs_apply(directory, workspace, body).await?;
        Ok(response.into_inner())
    }
}
