pub mod internal;

#[cfg(test)]
#[macro_use]
extern crate mockall;

use crate::internal::{handler::greeter::MyGreeter, repository::repository::GreeterRepoImpl, usecase::greeter::GreeterUsecaseImpl};

use tonic::transport::Server;

use crate::internal::handler::greeter::hello_world::greeter_server::GreeterServer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter_repo = Box::new(GreeterRepoImpl::new());
    let greeter_usecase = Box::new(GreeterUsecaseImpl::new(greeter_repo));
    let greeter = MyGreeter{
        greeter_usecase
    };

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
