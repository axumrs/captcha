use std::io::Cursor;

use axum::{http::header, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/captcha", get(captcha));

    let listener = TcpListener::bind(&"127.0.0.1:56789").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn captcha() -> impl IntoResponse {
    let (img, code) = captcha::generator().unwrap();
    let mut buf = vec![];
    img.write_to(&mut Cursor::new(&mut buf), captcha::ImageFormat::Png)
        .unwrap();
    println!("验证码：{code}");
    ([(header::CONTENT_TYPE, "image/png")], buf)
}
