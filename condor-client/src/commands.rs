#[derive(clap::Subcommand)]
pub enum Command {
    CreateJobGroup {
        name: String,
        template: String,
    },
    GetJobGroup {
        id: String,
    },
    CreateJob {
        group_id: String,
        name: String,
        #[arg(short = 'P', long = "param", value_name = "KEY=VALUE")]
        param: Vec<String>,
    },
    GetJob {
        id: String,
    },
    UpdateJobStatus {
        id: String,
        status: String,
    },
    RunJobs {
        group_id: String,
    },
    ListSessions {
        #[arg(long, default_value = "http://127.0.0.1:4096")]
        url: String,
    },
    ListMessages {
        #[arg(long, default_value = "http://127.0.0.1:4096")]
        url: String,
        #[arg(long)]
        session_id: String,
    },
}
