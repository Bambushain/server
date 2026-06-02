#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::firebase::start().await
}
