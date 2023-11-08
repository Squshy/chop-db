use chop_db::HashIndex;
use commander::commander_server::Commander;
use commander::{commander_server::CommanderServer, CommanderRequest, CommanderResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod commander {
    tonic::include_proto!("commander");
}

struct MyCommander {
    hash_index: HashIndex,
}

impl Default for MyCommander {
    fn default() -> Self {
        MyCommander {
            hash_index: HashIndex::new().unwrap(),
        }
    }
}

#[tonic::async_trait]
impl Commander for MyCommander {
    async fn get(
        &self,
        request: Request<CommanderRequest>,
    ) -> Result<Response<CommanderResponse>, Status> {
        let val = self.hash_index.get(&request.into_inner().value).unwrap();

        let reply = commander::CommanderResponse {
            successful: true,
            message: val.unwrap_or("No value".to_string()),
        };

        Ok(Response::new(reply))
    }

    async fn set(
        &self,
        request: Request<CommanderRequest>,
    ) -> Result<Response<CommanderResponse>, Status> {
        let reply = commander::CommanderResponse {
            successful: true,
            message: format!("hehe"),
        };

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<CommanderRequest>,
    ) -> Result<Response<CommanderResponse>, Status> {
        let reply = commander::CommanderResponse {
            successful: true,
            message: format!("hehe"),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let addr = "127.0.0.1:50051".parse()?;
    let commander = MyCommander::default();

    Server::builder()
        .add_service(CommanderServer::new(commander))
        .serve(addr)
        .await?;

    Ok(())
}
