#[actix::main]
async fn main() -> std::io::Result<()> {
    bamboo_backend::mailer_notifications::start().await
}
