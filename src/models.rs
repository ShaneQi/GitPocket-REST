#[derive(Serialize)]
pub struct Repo {
    pub id: Option<i32>,
    pub owner: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Host {
    pub id: Option<i32>,
    pub name: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct Tag {
    pub name: String,
}
