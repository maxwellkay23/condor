use clap::Parser;
use condor_common::CondorGrpcClient;
use condor_common::JobStatus;
use condor_common::OpencodeClient;

mod commands;
mod render;

use commands::Command;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::CreateJobGroup { name, template } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.create_job_group(name, template).await?;
            println!("create_job_group: id={}", response.id);
        }
        Command::GetJobGroup { id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.get_job_group(id).await?;
            println!(
                "get_job_group: id={}, name={}, template={}",
                response.id, response.name, response.template
            );
        }
        Command::CreateJob {
            group_id,
            name,
            param,
        } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let mut parameters = std::collections::HashMap::new();
            for p in &param {
                if let Some((k, v)) = p.split_once('=') {
                    parameters.insert(k.to_owned(), v.to_owned());
                }
            }
            let response = client.create_job(group_id, name, parameters).await?;
            println!("create_job: id={}", response.id);
        }
        Command::GetJob { id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.get_job(id).await?;
            let group = client.get_job_group(response.group_id.clone()).await?;

            println!(
                "get_job: id={}, group_id={}, name={}, status={:?}",
                response.id, response.group_id, response.name, response.status
            );

            let rendered = render::render_template(&group.template, &response.parameters)?;
            println!("--- rendered ---\n{rendered}\n----------------");
        }
        Command::UpdateJobStatus { id, status } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let status_val = match status.to_lowercase().as_str() {
                "created" => JobStatus::Created,
                "running" => JobStatus::Running,
                "failed" => JobStatus::Failed,
                "completed" => JobStatus::Completed,
                _ => {
                    eprintln!(
                        "unknown status: {status} (use created, running, failed, or completed)"
                    );
                    return Ok(());
                }
            };
            let response = client.update_job_status(id, status_val).await?;
            println!("update_job_status: success={}", response.success);
        }
        Command::RunJobs { group_id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let group = client.get_job_group(group_id.clone()).await?;

            let pop = client.pop_job_from_group(group_id).await?;
            if pop.job_id.is_empty() {
                println!("No jobs in group");
                return Ok(());
            }

            let job = client.get_job(pop.job_id.clone()).await?;
            let rendered = render::render_template(&group.template, &job.parameters)?;

            client
                .update_job_status(pop.job_id, JobStatus::Completed)
                .await?;

            let status = std::process::Command::new("opencode")
                .arg("run")
                .arg(&rendered)
                .status()?;
            if !status.success() {
                eprintln!("opencode run exited with: {}", status);
            }
        }
        Command::ListSessions { url } => {
            let client = OpencodeClient::new(url);
            let sessions = client.list_sessions().await?;
            for session in &sessions {
                println!("{} | {} | {}", &*session.id, session.slug, session.title);
            }
        }
        Command::ListMessages { url, session_id } => {
            use condor_common::openapi::types::{Message, Part, ToolState};

            let client = OpencodeClient::new(url);
            let messages = client.list_messages(&session_id).await?;
            for msg in &messages {
                let (id, role) = match &msg.info {
                    Message::UserMessage(m) => (&*m.id as &str, "user"),
                    Message::AssistantMessage(m) => (&*m.id as &str, "assistant"),
                };
                println!("--- {} ({}) ---", id, role);
                for part in &msg.parts {
                    match part {
                        Part::TextPart(t) => println!("{}", t.text),
                        Part::ReasoningPart(r) => println!("{}", r.text),
                        Part::ToolPart(t) => {
                            match &t.state {
                                ToolState::Completed(c) => println!("{}", c.output),
                                ToolState::Error(e) => println!("error: {}", e.error),
                                ToolState::Pending(_) => println!("[pending tool: {}]", t.tool),
                                ToolState::Running(_) => println!("[running tool: {}]", t.tool),
                            }
                        }
                        _ => {}
                    }
                }
                println!();
            }
        }
    }

    Ok(())
}
