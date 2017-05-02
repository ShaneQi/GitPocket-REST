#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub owner: String,
    pub name: String,
    pub host: Host,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub url: String,
}