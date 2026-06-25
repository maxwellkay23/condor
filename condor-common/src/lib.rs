pub use error::CondorError;
pub use opencode_error::CondorOpencodeError;

mod error;
mod opencode_error;

pub mod proto {
    tonic::include_proto!("condor_common");
}

mod grpc_client;
pub use grpc_client::CondorGrpcClient;

mod types;
pub use types::{Job, JobGroup, JobStatus};

mod opencode_client;
pub use opencode_client::{
    AppClient, AuthClient, CommandClient, ConfigClient, ExperimentalClient, FileClient, FindClient,
    FormatterClient, GlobalClient, InstanceClient, LspClient, McpClient, OpencodeClient,
    PathClient, PermissionClient, ProjectClient, ProviderClient, PtyClient, QuestionClient,
    SessionClient, SyncClient, TuiClient, V2Client, VcsClient,
};

mod render;
pub use render::render_template;

pub mod openapi {
    include!(concat!(env!("OUT_DIR"), "/openapi.rs"));
}
