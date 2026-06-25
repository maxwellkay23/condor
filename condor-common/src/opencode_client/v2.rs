use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct V2Client<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl V2Client<'_> {
    pub async fn session_list(
        &self,
        cursor: Option<&str>,
        directory: Option<&str>,
        limit: Option<f64>,
        order: Option<types::V2SessionListOrder>,
        path: Option<&str>,
        roots: Option<&types::V2SessionListRoots>,
        search: Option<&str>,
        start: Option<f64>,
        workspace: Option<&str>,
    ) -> Result<types::V2SessionsResponse, CondorOpencodeError> {
        let response = self
            .client
            .v2_session_list(
                cursor, directory, limit, order, path, roots, search, start, workspace,
            )
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_prompt(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::V2SessionPromptBody,
    ) -> Result<types::SessionMessage, CondorOpencodeError> {
        let id: types::V2SessionPromptSessionId = session_id.parse()?;
        let response = self
            .client
            .v2_session_prompt(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_compact(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<(), CondorOpencodeError> {
        let id: types::V2SessionCompactSessionId = session_id.parse()?;
        let response = self
            .client
            .v2_session_compact(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_wait(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<(), CondorOpencodeError> {
        let id: types::V2SessionWaitSessionId = session_id.parse()?;
        let response = self
            .client
            .v2_session_wait(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_context(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::SessionMessage>, CondorOpencodeError> {
        let id: types::V2SessionContextSessionId = session_id.parse()?;
        let response = self
            .client
            .v2_session_context(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_messages(
        &self,
        session_id: &str,
        cursor: Option<&str>,
        directory: Option<&str>,
        limit: Option<f64>,
        order: Option<types::V2SessionMessagesOrder>,
        workspace: Option<&str>,
    ) -> Result<types::V2SessionMessagesResponse, CondorOpencodeError> {
        let id: types::V2SessionMessagesSessionId = session_id.parse()?;
        let response = self
            .client
            .v2_session_messages(&id, cursor, directory, limit, order, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn model_list(
        &self,
        location: Option<&types::V2ModelListLocation>,
    ) -> Result<Vec<types::ModelV2Info>, CondorOpencodeError> {
        let response = self.client.v2_model_list(location).await?;
        Ok(response.into_inner())
    }

    pub async fn provider_list(
        &self,
        location: Option<&types::V2ProviderListLocation>,
    ) -> Result<Vec<types::ProviderV2Info>, CondorOpencodeError> {
        let response = self.client.v2_provider_list(location).await?;
        Ok(response.into_inner())
    }

    pub async fn provider_get(
        &self,
        provider_id: &str,
        location: Option<&types::V2ProviderGetLocation>,
    ) -> Result<types::ProviderV2Info, CondorOpencodeError> {
        let response = self.client.v2_provider_get(provider_id, location).await?;
        Ok(response.into_inner())
    }
}
