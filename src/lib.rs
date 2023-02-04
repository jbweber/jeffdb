use api::echo::echo_service_server::EchoService;
use api::echo::{EchoRequest, EchoResponse};
use tonic::{Request, Response, Status};

pub mod api;
pub mod log;

#[derive(Debug, Default)]
pub struct Echo {}

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = EchoResponse {
            message: format!("{}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}
