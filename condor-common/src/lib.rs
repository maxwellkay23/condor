pub use error::CondorError;

mod error;

pub mod proto {
    tonic::include_proto!("condor_common");
}

mod grpc_client;
pub use grpc_client::CondorGrpcClient;

mod types;
pub use types::{Job, JobGroup, JobStatus};

mod opencode_client;
pub use opencode_client::OpencodeClient;

pub mod openapi {
    include!(concat!(env!("OUT_DIR"), "/openapi.rs"));
}
