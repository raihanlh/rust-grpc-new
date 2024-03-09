pub trait GreeterRepository {
    fn say_hello(&self, name: &str) -> String;
}