use tree_walk_api::http;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    println!("->> LISTENING on {}", listener.local_addr()?);

    http::serve(listener)?.await
}
