#[derive(edgedb_derive::Queryable, serde::Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}
