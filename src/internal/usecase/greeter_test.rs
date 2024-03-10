#[cfg(test)]

use crate::internal::{repository::repository::MockGreeterRepository, usecase::greeter::GreeterUsecaseImpl};
use pretty_assertions as pa;

#[tokio::test]
async fn test_say_hello_success() {
  let mut mock_greeter_repo = MockGreeterRepository::default();

  mock_greeter_repo
    .expect_say_hello()
    .withf(move |name| {
      pa::assert_eq!(
        "test_name".to_string(),
        *name
    );

      true
    })
    .return_once(move |_| {
      "test_name".to_string()
    });

    let mut uc = GreeterUsecaseImpl::mock();
    uc.greeter_repo = Box::new(mock_greeter_repo);

    let expect = "test_name".to_string();

    let result = uc.greeter_repo.say_hello("test_name");

    pa::assert_eq!(expect, result);
}
