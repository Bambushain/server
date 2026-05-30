#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::sse::start_server().await
}
