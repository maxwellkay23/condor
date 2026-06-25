use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct SessionClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl SessionClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        limit: Option<f64>,
        path: Option<&str>,
        roots: Option<&types::SessionListRoots>,
        scope: Option<types::SessionListScope>,
        search: Option<&str>,
        start: Option<f64>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Session>, CondorOpencodeError> {
        let response = self
            .client
            .session_list(
                directory, limit, path, roots, scope, search, start, workspace,
            )
            .await?;
        Ok(response.into_inner())
    }

    pub async fn get(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_get(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn create(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionCreateBody,
    ) -> Result<types::Session, CondorOpencodeError> {
        let response = self
            .client
            .session_create(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn delete(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionDeleteSessionId = session_id.parse()?;
        let response = self
            .client
            .session_delete(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionUpdateBody,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionUpdateSessionId = session_id.parse()?;
        let response = self
            .client
            .session_update(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn messages(
        &self,
        session_id: &str,
        before: Option<&str>,
        directory: Option<&str>,
        limit: Option<i64>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::SessionMessagesResponseItem>, CondorOpencodeError> {
        let id: types::SessionMessagesSessionId = session_id.parse()?;
        let response = self
            .client
            .session_messages(&id, before, directory, limit, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn prompt(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionPromptBody,
    ) -> Result<types::SessionPromptResponse, CondorOpencodeError> {
        let id: types::SessionPromptSessionId = session_id.parse()?;
        let response = self
            .client
            .session_prompt(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn abort(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionAbortSessionId = session_id.parse()?;
        let response = self.client.session_abort(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn fork(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionForkBody,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionForkSessionId = session_id.parse()?;
        let response = self
            .client
            .session_fork(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn share(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionShareSessionId = session_id.parse()?;
        let response = self.client.session_share(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn unshare(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionUnshareSessionId = session_id.parse()?;
        let response = self
            .client
            .session_unshare(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn summarize(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionSummarizeBody,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionSummarizeSessionId = session_id.parse()?;
        let response = self
            .client
            .session_summarize(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn init(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionInitBody,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionInitSessionId = session_id.parse()?;
        let response = self
            .client
            .session_init(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn children(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Session>, CondorOpencodeError> {
        let id: types::SessionChildrenSessionId = session_id.parse()?;
        let response = self
            .client
            .session_children(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<std::collections::HashMap<String, types::SessionStatus>, CondorOpencodeError> {
        let response = self.client.session_status(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn todo(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Todo>, CondorOpencodeError> {
        let id: types::SessionTodoSessionId = session_id.parse()?;
        let response = self.client.session_todo(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn diff(
        &self,
        session_id: &str,
        directory: Option<&str>,
        message_id: Option<&types::SessionDiffMessageId>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::SnapshotFileDiff>, CondorOpencodeError> {
        let id: types::SessionDiffSessionId = session_id.parse()?;
        let response = self
            .client
            .session_diff(&id, directory, message_id, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn message(
        &self,
        session_id: &str,
        message_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::SessionMessageResponse, CondorOpencodeError> {
        let sid: types::SessionMessageSessionId = session_id.parse()?;
        let mid: types::SessionMessageMessageId = message_id.parse()?;
        let response = self
            .client
            .session_message(&sid, &mid, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn delete_message(
        &self,
        session_id: &str,
        message_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let sid: types::SessionDeleteMessageSessionId = session_id.parse()?;
        let mid: types::SessionDeleteMessageMessageId = message_id.parse()?;
        let response = self
            .client
            .session_delete_message(&sid, &mid, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn command(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionCommandBody,
    ) -> Result<types::SessionCommandResponse, CondorOpencodeError> {
        let id: types::SessionCommandSessionId = session_id.parse()?;
        let response = self
            .client
            .session_command(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn shell(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionShellBody,
    ) -> Result<types::SessionShellResponse, CondorOpencodeError> {
        let id: types::SessionShellSessionId = session_id.parse()?;
        let response = self
            .client
            .session_shell(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn revert(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionRevertBody,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionRevertSessionId = session_id.parse()?;
        let response = self
            .client
            .session_revert(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn unrevert(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Session, CondorOpencodeError> {
        let id: types::SessionUnrevertSessionId = session_id.parse()?;
        let response = self
            .client
            .session_unrevert(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn prompt_async(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::SessionPromptAsyncBody,
    ) -> Result<(), CondorOpencodeError> {
        let id: types::SessionPromptAsyncSessionId = session_id.parse()?;
        let response = self
            .client
            .session_prompt_async(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }
}
