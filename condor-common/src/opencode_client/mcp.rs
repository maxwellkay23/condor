use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct McpClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl McpClient<'_> {
    pub async fn status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<std::collections::HashMap<String, types::McpStatus>, CondorOpencodeError> {
        let response = self.client.mcp_status(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn add(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::McpAddBody,
    ) -> Result<std::collections::HashMap<String, types::McpStatus>, CondorOpencodeError> {
        let response = self.client.mcp_add(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn connect(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self.client.mcp_connect(name, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn disconnect(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .mcp_disconnect(name, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn auth_start(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::McpAuthStartResponse, CondorOpencodeError> {
        let response = self
            .client
            .mcp_auth_start(name, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn auth_callback(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::McpAuthCallbackBody,
    ) -> Result<types::McpStatus, CondorOpencodeError> {
        let response = self
            .client
            .mcp_auth_callback(name, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn auth_remove(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::McpAuthRemoveResponse, CondorOpencodeError> {
        let response = self
            .client
            .mcp_auth_remove(name, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn auth_authenticate(
        &self,
        name: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::McpStatus, CondorOpencodeError> {
        let response = self
            .client
            .mcp_auth_authenticate(name, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }
}
