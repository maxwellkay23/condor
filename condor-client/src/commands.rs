#[derive(clap::Subcommand)]
pub enum Command {
    CreateJobGroup {
        name: String,
        template: String,
    },
    GetJobGroup {
        id: String,
    },
    UpdateJobGroup {
        id: String,
        template: String,
    },
    ListJobGroups,
    DeleteJobGroup {
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
    ListJobs {
        group_id: String,
    },
    DeleteJob {
        id: String,
    },
    UpdateJobStatus {
        id: String,
        status: String,
    },
    RunJobs {
        group_id: String,
    },
}
