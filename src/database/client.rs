use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pg_pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pg_pool: Pool<Postgres>) -> Self {
        DBClient { pg_pool }
    }
}
