use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct SyncClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl SyncClient<'_> {
    pub async fn start(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.sync_start(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn replay(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SyncReplayBody,
    ) -> Result<types::SyncReplayResponse, CondorOpencodeError> {
        let response = self.client.sync_replay(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn steal(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SyncStealBody,
    ) -> Result<types::SyncStealResponse, CondorOpencodeError> {
        let response = self.client.sync_steal(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn history_list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &std::collections::HashMap<String, u64>,
    ) -> Result<Vec<types::SyncHistoryListResponseItem>, CondorOpencodeError> {
        let response = self
            .client
            .sync_history_list(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }
}
