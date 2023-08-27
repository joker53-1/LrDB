#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
#[allow(deprecated)]
pub mod protos {
   include!(concat!(env!("OUT_DIR"), "/mod.rs"));
   // tonic::include_proto!("coprocessor");
   // tonic::include_proto!("eraftpb");
   // tonic::include_proto!("errorpb");
   // tonic::include_proto!("kvrpcpb");
   // tonic::include_proto!("metapb");
   // tonic::include_proto!("raft_cmdpb");
   // tonic::include_proto!("raft_serverpb");
   // tonic::include_proto!("schedulerpb");
   // tonic::include_proto!("tinykvpb");
}
