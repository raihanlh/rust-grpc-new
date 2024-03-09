use super::usecase::GreeterUsecase;

// Implement the MyTrait trait for a struct or type
pub struct GreeterUsecaseImpl;

impl GreeterUsecaseImpl {
    pub fn new() -> Self {
        GreeterUsecaseImpl{}
    }
}

impl GreeterUsecase for GreeterUsecaseImpl {
    fn say_hello(&self, name: &str) -> String {
        println!("This is another method implementation");
        return format!("{}", name);
    }
}

