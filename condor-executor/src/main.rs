use clap::Parser;
use condor_common::CondorGrpcClient;
use condor_common::JobStatus;
use condor_common::OpencodeClient;
use condor_common::openapi::types;
use condor_common::render_template;
use serde::Deserialize;
use tokio::time::{Duration, sleep, timeout};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    config: String,
}

#[derive(Deserialize)]
struct Config {
    condor_server_url: String,
    opencode_server_url: String,
    timeout: u64,
    poll_interval: u64,
    groups: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let contents = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    let mut client = CondorGrpcClient::connect(&config.condor_server_url).await?;

    println!("condor-executor starting");
    println!("  condor_server_url: {}", config.condor_server_url);
    println!("  opencode_server_url: {}", config.opencode_server_url);
    println!("  timeout: {}s", config.timeout);
    println!("  poll_interval: {}s", config.poll_interval);
    println!("  groups: {:?}", config.groups);

    loop {
        let mut any_job_processed = false;

        for group_id in &config.groups {
            let pop = client.pop_job_from_group(group_id.clone()).await?;
            if pop.job_id.is_empty() {
                continue;
            }

            any_job_processed = true;
            let job_id = pop.job_id;

            let job = client.get_job(job_id.clone()).await?;

            client
                .update_job_status(job_id.clone(), JobStatus::Running, group_id.clone())
                .await?;

            let template = client
                .get_group_template(group_id.clone(), job.template_version)
                .await?
                .template;

            let rendered = match render_template(&template, &job.parameters) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("render error for job {}: {}", job_id, e);
                    client
                        .update_job_status(job_id, JobStatus::Failed, group_id.clone())
                        .await?;
                    continue;
                }
            };

            let result =
                run_opencode_job(&config.opencode_server_url, &rendered, config.timeout).await;

            match result {
                Ok(()) => {
                    client
                        .update_job_status(job_id.clone(), JobStatus::Completed, group_id.clone())
                        .await?;
                    println!("job {} completed successfully", job_id);
                }
                Err(e) => {
                    eprintln!("job {} failed: {}", job_id, e);
                    client
                        .update_job_status(job_id, JobStatus::Failed, group_id.clone())
                        .await?;
                }
            }
        }

        if !any_job_processed {
            sleep(Duration::from_secs(config.poll_interval)).await;
        }
    }
}

async fn run_opencode_job(
    opencode_server_url: &str,
    prompt: &str,
    timeout_secs: u64,
) -> Result<(), String> {
    let inner = condor_common::openapi::Client::new(opencode_server_url);
    let oc = OpencodeClient::new(&inner);

    let session = oc
        .session
        .create(None, None, &Default::default())
        .await
        .map_err(|e| format!("session create: {}", e))?;

    let session_id = session.id.to_string();
    println!("session: {}", session.slug);
    println!("Input: {}", prompt);

    let part = types::SessionPromptBodyPartsItem::TextPartInput(types::TextPartInput {
        id: None,
        ignored: None,
        metadata: Default::default(),
        synthetic: None,
        text: prompt.to_owned(),
        time: None,
        type_: types::TextPartInputType::Text,
    });
    let body = types::SessionPromptBody {
        agent: None,
        format: None,
        message_id: None,
        model: None,
        no_reply: Some(false),
        parts: vec![part],
        system: None,
        tools: Default::default(),
        variant: None,
    };

    let result = timeout(
        Duration::from_secs(timeout_secs),
        oc.session.prompt(&session_id, None, None, &body),
    )
    .await;

    let response = match result {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => {
            let _ = oc.session.abort(&session_id, None, None).await;
            return Err(format!("opencode prompt: {}", e));
        }
        Err(_) => {
            let _ = oc.session.abort(&session_id, None, None).await;
            return Err("timed out".to_owned());
        }
    };

    for part in &response.parts {
        match part {
            types::Part::TextPart(t) => {
                if !t.text.is_empty() {
                    println!("Output: {}", t.text);
                }
            }
            types::Part::ReasoningPart(r) => {
                if !r.text.is_empty() {
                    println!("Thought: {}", r.text);
                }
            }
            types::Part::ToolPart(t) => match &t.state {
                types::ToolState::Completed(c) => {
                    let trimmed = c.output.trim();
                    if !trimmed.is_empty() {
                        println!("[{}] {}", t.tool, trimmed);
                    }
                }
                types::ToolState::Error(e) => {
                    let trimmed = e.error.trim();
                    if !trimmed.is_empty() {
                        eprintln!("[{} error] {}", t.tool, trimmed);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
