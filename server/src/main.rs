use chop_db::HashIndex;
use lumberjack::lumberjack_server::Lumberjack;
use lumberjack::{lumberjack_server::LumberjackServer, LumberjackRequest, LumberjackResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod lumberjack {
    tonic::include_proto!("lumberjack");
}

struct MyLumberjack {
    hash_index: HashIndex,
}

impl Default for MyLumberjack {
    fn default() -> Self {
        MyLumberjack {
            hash_index: HashIndex::new().unwrap(),
        }
    }
}

#[tonic::async_trait]
impl Lumberjack for MyLumberjack {
    async fn get(
        &self,
        request: Request<LumberjackRequest>,
    ) -> Result<Response<LumberjackResponse>, Status> {
        let val = self.hash_index.get(&request.into_inner().value).unwrap();

        let reply = lumberjack::LumberjackResponse {
            successful: true,
            message: val.unwrap_or("No value".to_string()),
        };

        Ok(Response::new(reply))
    }

    async fn set(
        &self,
        request: Request<LumberjackRequest>,
    ) -> Result<Response<LumberjackResponse>, Status> {
        let reply = lumberjack::LumberjackResponse {
            successful: true,
            message: format!("hehe"),
        };

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<LumberjackRequest>,
    ) -> Result<Response<LumberjackResponse>, Status> {
        let reply = lumberjack::LumberjackResponse {
            successful: true,
            message: format!("hehe"),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let addr = "127.0.0.1:50051".parse()?;
    let lumberjack = MyLumberjack::default();

    Server::builder()
        .add_service(LumberjackServer::new(lumberjack))
        .serve(addr)
        .await?;

    Ok(())
}
