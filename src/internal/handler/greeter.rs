use tonic::{Request, Response, Status};

use hello_world::greeter_server::Greeter;
use hello_world::{HelloReply, HelloRequest};
use crate::internal::usecase::usecase::GreeterUsecase;


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct MyGreeter {
    pub greeter_usecase: Box<dyn GreeterUsecase + Send + Sync>,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let res = self.greeter_usecase.say_hello(request.into_inner().name.as_str());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", res),
        };
        Ok(Response::new(reply))
    }
}