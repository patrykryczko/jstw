use maud::{DOCTYPE, Markup, html};

pub fn page_header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        script src="https://unpkg.com/htmx.org@2.0.4" {}
    }
}
