use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct PtyClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl PtyClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::Pty>, CondorOpencodeError> {
        let response = self.client.pty_list(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn create(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::PtyCreateBody,
    ) -> Result<types::Pty, CondorOpencodeError> {
        let response = self.client.pty_create(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn get(
        &self,
        pty_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::Pty, CondorOpencodeError> {
        let id: types::PtyGetPtyId = pty_id.parse()?;
        let response = self.client.pty_get(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &self,
        pty_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::PtyUpdateBody,
    ) -> Result<types::Pty, CondorOpencodeError> {
        let id: types::PtyUpdatePtyId = pty_id.parse()?;
        let response = self
            .client
            .pty_update(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn remove(
        &self,
        pty_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::PtyRemovePtyId = pty_id.parse()?;
        let response = self.client.pty_remove(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn connect(
        &self,
        pty_id: &str,
        cursor: Option<&str>,
        directory: Option<&str>,
        ticket: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::PtyConnectPtyId = pty_id.parse()?;
        let response = self
            .client
            .pty_connect(&id, cursor, directory, ticket, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn connect_token(
        &self,
        pty_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<types::PtyConnectTokenResponse, CondorOpencodeError> {
        let id: types::PtyConnectTokenPtyId = pty_id.parse()?;
        let response = self
            .client
            .pty_connect_token(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn shells(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::PtyShellsResponseItem>, CondorOpencodeError> {
        let response = self.client.pty_shells(directory, workspace).await?;
        Ok(response.into_inner())
    }
}
