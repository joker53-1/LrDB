use protos::protos::{
    coprocessor,
    kvrpcpb::{
        BatchRollbackRequest, BatchRollbackResponse, CheckTxnStatusRequest, CheckTxnStatusResponse,
        CommitRequest, CommitResponse, GetRequest, GetResponse, PrewriteRequest, PrewriteResponse,
        RawDeleteRequest, RawDeleteResponse, RawGetRequest, RawGetResponse, RawPutRequest,
        RawPutResponse, RawScanRequest, RawScanResponse, ResolveLockRequest, ResolveLockResponse,
        ScanRequest, ScanResponse,
    },
    raft_serverpb::{Done, RaftMessage, SnapshotChunk},
    tinykvpb::tiny_kv_server::TinyKv,
    tinykvpb::tiny_kv_server::TinyKvServer,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct Server;

impl Server {
    pub async fn start(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr = addr.parse().unwrap();
        let server = Server::default();

        tonic::transport::Server::builder()
            .add_service(TinyKvServer::new(server))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl TinyKv for Server {
    async fn kv_get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = GetResponse {
            region_error: None,
            error: None,
            value: request.into_inner().key,
            not_found: false,
        };
        Ok(Response::new(reply))
    }

    async fn kv_scan(
        &self,
        _request: tonic::Request<ScanRequest>,
    ) -> std::result::Result<tonic::Response<ScanResponse>, tonic::Status> {
        todo!()
    }

    async fn kv_prewrite(
        &self,
        _request: tonic::Request<PrewriteRequest>,
    ) -> std::result::Result<tonic::Response<PrewriteResponse>, tonic::Status> {
        todo!()
    }

    async fn kv_commit(
        &self,
        _request: tonic::Request<CommitRequest>,
    ) -> std::result::Result<tonic::Response<CommitResponse>, tonic::Status> {
        todo!()
    }

    async fn kv_check_txn_status(
        &self,
        _request: tonic::Request<CheckTxnStatusRequest>,
    ) -> std::result::Result<tonic::Response<CheckTxnStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn kv_batch_rollback(
        &self,
        _request: tonic::Request<BatchRollbackRequest>,
    ) -> std::result::Result<tonic::Response<BatchRollbackResponse>, tonic::Status> {
        todo!()
    }

    async fn kv_resolve_lock(
        &self,
        _request: tonic::Request<ResolveLockRequest>,
    ) -> std::result::Result<tonic::Response<ResolveLockResponse>, tonic::Status> {
        todo!()
    }

    /// RawKV commands.
    async fn raw_get(
        &self,
        _request: tonic::Request<RawGetRequest>,
    ) -> std::result::Result<tonic::Response<RawGetResponse>, tonic::Status> {
        todo!()
    }

    async fn raw_put(
        &self,
        _request: tonic::Request<RawPutRequest>,
    ) -> std::result::Result<tonic::Response<RawPutResponse>, tonic::Status> {
        todo!()
    }

    async fn raw_delete(
        &self,
        _request: tonic::Request<RawDeleteRequest>,
    ) -> std::result::Result<tonic::Response<RawDeleteResponse>, tonic::Status> {
        todo!()
    }

    async fn raw_scan(
        &self,
        _request: tonic::Request<RawScanRequest>,
    ) -> std::result::Result<tonic::Response<RawScanResponse>, tonic::Status> {
        todo!()
    }

    /// Raft commands (tinykv <-> tinykv).
    async fn raft(
        &self,
        _request: tonic::Request<tonic::Streaming<RaftMessage>>,
    ) -> std::result::Result<tonic::Response<Done>, tonic::Status> {
        todo!()
    }

    async fn snapshot(
        &self,
        _request: tonic::Request<tonic::Streaming<SnapshotChunk>>,
    ) -> std::result::Result<tonic::Response<Done>, tonic::Status> {
        todo!()
    }

    /// Coprocessor
    async fn coprocessor(
        &self,
        _request: tonic::Request<coprocessor::Request>,
    ) -> std::result::Result<tonic::Response<coprocessor::Response>, tonic::Status> {
        todo!()
    }
}
