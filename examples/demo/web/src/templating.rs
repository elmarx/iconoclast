use askama::Template;
use axum::response::{Html, IntoResponse};
use axum_extra::response::InternalServerError;

pub fn init() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(index))
}

async fn index() -> Result<impl IntoResponse, InternalServerError<askama::Error>> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        name: String,
    }

    let template = Tmpl {
        name: "World".to_string(),
    };
    Ok(Html(template.render().map_err(InternalServerError)))
}
