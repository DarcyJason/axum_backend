use crate::database::client::DBClient;

pub trait UserRepository {}

impl UserRepository for DBClient {}
