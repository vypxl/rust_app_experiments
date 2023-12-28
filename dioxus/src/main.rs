use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listen_addr: std::net::SocketAddr = ([127, 0, 0, 1], 8080).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> <title>Dioxus LiveView with Axum</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
                </html>
                "#,
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{listen_addr}/ws"))
                ))
            }),
        )
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );
    println!("Listening on {listen_addr}");

    axum::Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! { div { "Hello World" }})
}
