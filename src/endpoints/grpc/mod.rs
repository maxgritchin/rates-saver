pub mod bidask;
pub mod ratessaver;
pub mod server;

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("proto_descriptor");
