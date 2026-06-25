use crate::CondorOpencodeError;
use crate::openapi::types;

pub struct QuestionClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl QuestionClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<types::QuestionRequest>, CondorOpencodeError> {
        let response = self.client.question_list(directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn reply(
        &self,
        request_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
        body: &types::QuestionReplyBody,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::QuestionReplyRequestId = request_id.parse()?;
        let response = self
            .client
            .question_reply(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn reject(
        &self,
        request_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::QuestionRejectRequestId = request_id.parse()?;
        let response = self
            .client
            .question_reject(&id, directory, workspace)
            .await?;
        Ok(response.into_inner())
    }
}
