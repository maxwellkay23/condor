use std::convert::TryFrom as _;

pub struct OpencodeClient {
    inner: crate::openapi::Client,
}

impl OpencodeClient {
    pub fn new(host: String) -> Self {
        let inner = crate::openapi::Client::new(&host);
        Self { inner }
    }

    pub async fn list_sessions(
        &self,
    ) -> Result<Vec<crate::openapi::types::Session>, Box<dyn std::error::Error>> {
        let response = self.inner.session_list(None, None, None, None, None, None, None, None).await?;
        Ok(response.into_inner())
    }

    pub async fn list_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<crate::openapi::types::SessionMessagesResponseItem>, Box<dyn std::error::Error>> {
        let id = crate::openapi::types::SessionMessagesSessionId::try_from(session_id)?;
        let response = self.inner.session_messages(&id, None, None, None, None).await?;
        Ok(response.into_inner())
    }
}
