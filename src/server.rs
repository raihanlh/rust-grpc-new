pub mod internal;

use crate::internal::{handler::greeter::MyGreeter, repository::greeter::GreeterRepoImpl};

use tonic::transport::Server;

use crate::internal::handler::greeter::hello_world::greeter_server::GreeterServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter{
        greeter_repo: Box::new(GreeterRepoImpl::new())
    };

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
