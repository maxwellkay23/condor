use clap::Parser;
use condor_common::CondorGrpcClient;
use condor_common::JobStatus;

mod commands;

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
                "get_job_group: id={}, name={}, template={}, current_version={}",
                response.id, response.name, response.template, response.current_version
            );
        }
        Command::UpdateJobGroup { id, template } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.update_job_group(id, template).await?;
            println!("update_job_group: new_version={}", response.new_version);
        }
        Command::ListJobGroups => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.list_job_groups().await?;
            for g in &response.job_groups {
                println!(
                    "  id={}, name={}, current_version={}",
                    g.id, g.name, g.current_version
                );
            }
            if response.job_groups.is_empty() {
                println!("no job groups");
            }
        }
        Command::DeleteJobGroup { id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            client.delete_job_group(id).await?;
            println!("delete_job_group: ok");
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

            println!(
                "get_job: id={}, group_id={}, name={}, status={:?}, template_version={}",
                response.id,
                response.group_id,
                response.name,
                response.status,
                response.template_version
            );

            println!("--- parameters ---");
            for (k, v) in &response.parameters {
                println!("  {k}={v}");
            }
        }
        Command::ListJobs { group_id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            let response = client.list_jobs(group_id).await?;
            for j in &response.jobs {
                println!(
                    "  id={}, name={}, status={:?}, template_version={}",
                    j.id, j.name, j.status, j.template_version
                );
            }
            if response.jobs.is_empty() {
                println!("no jobs");
            }
        }
        Command::DeleteJob { id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;
            client.delete_job(id).await?;
            println!("delete_job: ok");
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
            let response = client
                .update_job_status(id, status_val, String::new())
                .await?;
            println!("update_job_status: success={}", response.success);
        }
        Command::RunJobs { group_id } => {
            let addr = format!("http://{}:{}", args.host, args.port);
            let mut client = CondorGrpcClient::connect(&addr).await?;

            let pop = client.pop_job_from_group(group_id.clone()).await?;
            if pop.job_id.is_empty() {
                println!("No jobs in group");
                return Ok(());
            }

            let job = client.get_job(pop.job_id.clone()).await?;

            let template = client
                .get_group_template(group_id.clone(), job.template_version)
                .await?
                .template;

            let rendered = condor_common::render_template(&template, &job.parameters)?;

            client
                .update_job_status(pop.job_id.clone(), JobStatus::Running, group_id.clone())
                .await?;

            let status = std::process::Command::new("opencode")
                .arg("run")
                .arg(&rendered)
                .status()?;

            if status.success() {
                client
                    .update_job_status(pop.job_id, JobStatus::Completed, group_id.clone())
                    .await?;
                println!("job {} completed successfully", job.id);
            } else {
                eprintln!("opencode run exited with: {}", status);
                client
                    .update_job_status(pop.job_id, JobStatus::Failed, group_id.clone())
                    .await?;
            }
        }
    }

    Ok(())
}
