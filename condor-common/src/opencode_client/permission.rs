use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct PermissionClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl PermissionClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::PermissionRequest>, CondorOpencodeError> {
        let response = self.client.permission_list(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn respond(
        &self,
        session_id: &str,
        permission_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::PermissionRespondBody,
    ) -> Result<bool, CondorOpencodeError> {
        let sid: types::PermissionRespondSessionId = session_id.parse()?;
        let pid: types::PermissionRespondPermissionId = permission_id.parse()?;
        let response = self
            .client
            .permission_respond(&sid, &pid, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }
}
