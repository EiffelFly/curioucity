#[derive(edgedb_derive::Queryable, serde::Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

#[derive(edgedb_derive::Queryable, serde::Deserialize, Debug)]
pub struct Url {
    pub url: String,
    pub references: Vec<Url>,
}
