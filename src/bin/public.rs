#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_frontend::public::start_server().await
}
