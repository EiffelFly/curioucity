use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Website {
    pub content: String,
}
