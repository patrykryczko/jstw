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

impl Bookmark {
    pub fn new(
        id: i64,
        created_at: f64,
        edited_at: Option<f64>,
        deleted_at: Option<f64>,
        url: String,
        title: Option<String>,
        description: Option<String>,
        img_url: Option<String>,
    ) -> Bookmark {
        Bookmark {
            id,
            created_at,
            edited_at,
            deleted_at,
            url,
            title,
            description,
            img_url,
        }
    }
}
