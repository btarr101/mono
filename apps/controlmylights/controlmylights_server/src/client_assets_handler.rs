use axum::{
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};
use percent_encoding::percent_decode_str;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../ControlMyLightsClient/dist"]
struct ClientAssets;

const INDEX_HTML: &str = "index.html";

pub async fn client_assets_handler(uri: Uri) -> impl IntoResponse {
    let raw_path = uri.path().trim_start_matches('/');
    let decoded_path = percent_decode_str(raw_path).decode_utf8_lossy();
    let path = decoded_path.as_ref();

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    match ClientAssets::get(path) {
        None => {
            if path.contains(".") {
                not_found()
            } else {
                index_html()
            }
        }
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
    }
}

fn index_html() -> Response {
    match ClientAssets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response { (StatusCode::NOT_FOUND, "404").into_response() }
