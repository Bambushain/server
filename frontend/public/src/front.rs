use crate::utils::page;
use actix_web::Responder;
use maud::html;

pub async fn homepage() -> impl Responder {
    page(
        "Bambushain",
        html! {
            div."bamboo-banner__container is--home" {
                img."bamboo-banner is--home" src="/static/background.webp" {}
                div."bamboo-banner__title is--home" {
                    span."bamboo-banner__title is--home is--primary" { "Bambushain" }
                    span."bamboo-banner__title is--home is--sub" { "Der Ort wo Pandas sich zum Zocken treffen" }
                }
            }
            main."bamboo-page__content" {
                p {
                    "Mit Bambushain kannst du alle deine Final Fantasy XIV-Charaktere auf einmal verwalten. Du kannst ihre Jobs verwalten, die verschiedenen Klassen und selbst verständlich auch deine Unterkünfte."
                }
                p {
                    "Du bist in einer Static? Großartig, ihr könnt euch zusammen tun und gemeinsam einen Hain anlegen. In einem Hain habt ihr einen gemeinsamen Kalender in dem ihr alle eure Raids planen. Ihr könnt euch per Email über anstehende Raids benachrichtigen lassen."
                }
            }
        },
    )
}
