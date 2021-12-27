use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LinksDto {
    pub id: Option<String>,
    pub url: String,
}

#[derive(Debug)]
pub struct Links {
    pub id: String,
    pub url: String,
}