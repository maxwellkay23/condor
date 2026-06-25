use serde::Deserialize;

#[derive(Deserialize)]
pub struct NatsConfig {
    pub url: String,
    pub token: Option<String>,
}

pub struct NatsPublisher {
    client: async_nats::Client,
}

impl NatsPublisher {
    pub async fn connect(config: &NatsConfig) -> Result<Self, async_nats::ConnectError> {
        let client = if let Some(token) = &config.token {
            async_nats::ConnectOptions::new()
                .token(token.clone())
                .connect(&config.url)
                .await?
        } else {
            async_nats::connect(&config.url).await?
        };
        Ok(Self { client })
    }

    pub async fn publish_job_completed(&self, group_id: &str, job_id: &str) {
        let subject = format!("condor.{group_id}.job.completed");
        let payload = serde_json::json!({
            "job_id": job_id,
            "group_id": group_id,
            "status": "Completed",
        });
        if let Err(e) = self
            .client
            .publish(subject, payload.to_string().into())
            .await
        {
            println!("nats publish error: {e}");
        }
    }
}
