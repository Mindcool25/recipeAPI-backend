use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub ingredients: String,
    pub instructions: String,
    pub notes: String,
}
