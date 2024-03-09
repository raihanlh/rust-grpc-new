pub trait GreeterUsecase {
    fn say_hello(&self, name: &str) -> String;
}