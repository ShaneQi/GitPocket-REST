#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub id: Option<i64>,
    pub owner: String,
    pub name: String,
    pub host_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub id: Option<i64>,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}
