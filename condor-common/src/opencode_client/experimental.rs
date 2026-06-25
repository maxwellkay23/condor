use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct ExperimentalClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl ExperimentalClient<'_> {
    pub async fn console_get(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::ConsoleState, CondorOpencodeError> {
        let response = self
            .client
            .experimental_console_get(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn console_list_orgs(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::ExperimentalConsoleListOrgsResponse, CondorOpencodeError> {
        let response = self
            .client
            .experimental_console_list_orgs(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn console_switch_org(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ExperimentalConsoleSwitchOrgBody,
    ) -> Result<bool, CondorOpencodeError> {
        let response = self
            .client
            .experimental_console_switch_org(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn resource_list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<std::collections::HashMap<String, types::McpResource>, CondorOpencodeError> {
        let response = self
            .client
            .experimental_resource_list(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn session_list(
        &self,
        archived: Option<&types::ExperimentalSessionListArchived>,
        cursor: Option<f64>,
        directory: Option<&str>,
        limit: Option<f64>,
        roots: Option<&types::ExperimentalSessionListRoots>,
        search: Option<&str>,
        start: Option<f64>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::GlobalSession>, CondorOpencodeError> {
        let response = self
            .client
            .experimental_session_list(
                archived, cursor, directory, limit, roots, search, start, workspace,
            )
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Workspace>, CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_list(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_create(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ExperimentalWorkspaceCreateBody,
    ) -> Result<types::Workspace, CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_create(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_remove(
        &self,
        id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Workspace, CondorOpencodeError> {
        let id_: types::ExperimentalWorkspaceRemoveId = id.parse()?;
        let response = self
            .client
            .experimental_workspace_remove(&id_, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_status(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::ExperimentalWorkspaceStatusResponseItem>, CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_status(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_sync_list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<(), CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_sync_list(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_warp(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::ExperimentalWorkspaceWarpBody,
    ) -> Result<(), CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_warp(directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn workspace_adapter_list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::ExperimentalWorkspaceAdapterListResponseItem>, CondorOpencodeError> {
        let response = self
            .client
            .experimental_workspace_adapter_list(directory, workspace)
            .await?;
        Ok(response.into_inner())
    }
}
