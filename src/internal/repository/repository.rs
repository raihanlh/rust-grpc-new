use tonic::async_trait;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait GreeterRepository {
    fn say_hello(&self, name: &str) -> String;
}

pub struct GreeterRepoImpl;