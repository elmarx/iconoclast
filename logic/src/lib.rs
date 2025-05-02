//! Here lives the application logic, the business logic.
//! This is probably where most implementation work will be done.
pub mod hello;

#[expect(dead_code)]
#[derive(thiserror::Error, Debug)]
enum Error {}
