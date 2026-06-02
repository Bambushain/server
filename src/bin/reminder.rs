#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::reminder::start().await
}
