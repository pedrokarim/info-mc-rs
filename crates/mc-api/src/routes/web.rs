use axum::extract::Path;
use axum::http::header;
use axum::response::{Html, IntoResponse};

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../web/index.html"))
}

pub async fn css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../web/app.css"),
    )
}

pub async fn js() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        include_str!("../../web/app.js"),
    )
}

pub async fn asset(Path(name): Path<String>) -> impl IntoResponse {
    match name.as_str() {
        "originrealms-hero.png" => (
            [(header::CONTENT_TYPE, "image/png")],
            include_bytes!("../../../../docs/ref-originrealms.png").as_slice(),
        )
            .into_response(),
        "chunklock-news.png" => (
            [(header::CONTENT_TYPE, "image/png")],
            include_bytes!("../../../../docs/ref-chunklock.png").as_slice(),
        )
            .into_response(),
        _ => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}
