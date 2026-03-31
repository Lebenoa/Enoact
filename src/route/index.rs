use axum::response::{Html, IntoResponse};
use std::fs;

// '/'
pub async fn index() -> impl IntoResponse {
    #[cfg(debug_assertions)]
    {
        let html_content = fs::read_to_string("./web-ui/build/index.html")
            .expect("Failed to read index.html during development");
        Html(html_content)
    }

    #[cfg(not(debug_assertions))]
    {
        const INDEX_HTML: &'static str = include_str!("../../web-ui/build/index.html");
        Html(INDEX_HTML)
    }
}
