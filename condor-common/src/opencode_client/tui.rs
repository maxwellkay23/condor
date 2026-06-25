use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct TuiClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl TuiClient<'_> {
    pub async fn append_prompt(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::TuiAppendPromptBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .tui_append_prompt(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn open_help(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_open_help(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn open_sessions(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_open_sessions(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn open_themes(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_open_themes(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn open_models(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_open_models(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn submit_prompt(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_submit_prompt(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn clear_prompt(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_clear_prompt(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn execute_command(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::TuiExecuteCommandBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .tui_execute_command(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn show_toast(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::TuiShowToastBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .tui_show_toast(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn publish(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::EventTuiPromptAppend,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.tui_publish(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn select_session(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::TuiSelectSessionBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .tui_select_session(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn control_next(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::TuiControlNextResponse, CondorOpencodeError> {
        let response = self.client.tui_control_next(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn control_response(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &serde_json::Value,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .tui_control_response(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }
}
