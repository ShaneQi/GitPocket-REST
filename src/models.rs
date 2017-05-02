#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub id: Option<i32>,
    pub owner: String,
    pub name: String,

    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<Host>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub id: Option<i32>,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}
