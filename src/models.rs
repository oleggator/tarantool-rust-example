use tarantool::tuple::AsTuple;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub user_id: i32,
    pub company_id: i32,
    pub username: String,
    pub country: String,
}

impl AsTuple for User {}
