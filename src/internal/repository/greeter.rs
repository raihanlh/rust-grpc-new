use super::repository::{GreeterRepository, GreeterRepoImpl};

impl GreeterRepoImpl {
    pub fn new() -> Self {
        GreeterRepoImpl{}
    }
}

impl GreeterRepository for GreeterRepoImpl {
    fn say_hello(&self, name: &str) -> String {
        println!("This is another method implementation");
        return format!("{}", name);
    }
}

