use chop_db::HashIndex;
use forester::forester_message::ResponseStatus;
use forester::forester_server::Forester;
use forester::ForesterMessage;
use forester::{
    forester_server::ForesterServer, ForesterDeleteRequest, ForesterDeleteResponse,
    ForesterGetRequest, ForesterGetResponse, ForesterSetRequest, ForesterSetResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod forester {
    tonic::include_proto!("forester");
}

struct MyForester {
    hash_index: HashIndex,
}

impl Default for MyForester {
    fn default() -> Self {
        MyForester {
            hash_index: HashIndex::new().unwrap(),
        }
    }
}

#[tonic::async_trait]
impl Forester for MyForester {
    async fn get(
        &self,
        request: Request<ForesterGetRequest>,
    ) -> Result<Response<ForesterGetResponse>, Status> {
        let value = self.hash_index.get(&request.into_inner().key).unwrap();

        let reply = forester::ForesterGetResponse {
            message: Some(ForesterMessage {
                status: ResponseStatus::Success.into(),
                message: "Successful get.".to_string(),
            }),
            value,
        };

        Ok(Response::new(reply))
    }

    async fn set(
        &self,
        request: Request<ForesterSetRequest>,
    ) -> Result<Response<ForesterSetResponse>, Status> {
        let reply = forester::ForesterSetResponse {
            message: Some(ForesterMessage {
                status: ResponseStatus::Success.into(),
                message: "Successful set.".to_string(),
            }),
        };

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<ForesterDeleteRequest>,
    ) -> Result<Response<ForesterDeleteResponse>, Status> {
        let reply = forester::ForesterDeleteResponse {
            message: Some(ForesterMessage {
                status: ResponseStatus::Success.into(),
                message: "Successful delete.".to_string(),
            }),
            deleted: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let addr = "127.0.0.1:50051".parse()?;
    let forester = MyForester::default();

    Server::builder()
        .add_service(ForesterServer::new(forester))
        .serve(addr)
        .await?;

    Ok(())
}
