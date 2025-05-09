use serde::Serialize;

#[derive(Serialize)]
pub struct Bookmark {
    pub id: i64,
    pub created_at: f64,
    pub edited_at: Option<f64>,
    pub deleted_at: Option<f64>,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub img_url: Option<String>,
}
