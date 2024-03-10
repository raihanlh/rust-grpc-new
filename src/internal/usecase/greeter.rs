
use crate::internal::repository::repository::GreeterRepository;
use super::usecase::GreeterUsecase;

pub struct GreeterUsecaseImpl {
    pub greeter_repo: Box<dyn GreeterRepository + Send + Sync>,
}

impl GreeterUsecaseImpl {
    #[cfg(test)]
    pub fn mock() -> Self {
        use crate::internal::repository::repository::MockGreeterRepository;

        GreeterUsecaseImpl {
            greeter_repo: Box::new(MockGreeterRepository::default()),
        }
    }
    
    pub fn new(greeter_repo: Box<dyn GreeterRepository + Send + Sync>) -> Self {
        GreeterUsecaseImpl{
            greeter_repo
        }
    }
}

impl GreeterUsecase for GreeterUsecaseImpl {
    fn say_hello(&self, name: &str) -> String {
        println!("This is usecase method implementation");

        let res = self.greeter_repo.say_hello(name);

        return format!("Hello {}!", res);
    }
}

