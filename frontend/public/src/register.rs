use crate::utils::banner_page;
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpResponse, Responder};
use bamboo_common::backend::actix::cookie;
use bamboo_common::backend::dbal;
use bamboo_common::backend::services::DbConnection;
use bamboo_common::core::entities::User;
use bamboo_common::core::error::BambooError;
use base64::Engine;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};

fn register_form(exists: bool, name: &str, email: &str) -> Markup {
    banner_page(
        "Werde ein Panda",
        html! {
            h1 {
                "Registriere dich für Bambushain"
            }
            p {
                "Hier kannst du dich für Bambushain registrieren. Gib einfach deine Emailadresse ein und du bekommst einen Link zum Registrieren."
            }
            @if exists {
                p.bamboo-info {
                    "So wie es aussieht hast du schon einen Account. Wenn du dich unter diesem Account anmelden möchtest, klick einfach "
                    a href="/authentication/login" { "hier" }
                    " und du kannst dich direkt einloggen oder dein Passwort zurücksetzen."
                }
            }
            form.bamboo-form action="/register" method="post" {
                label.bamboo-label for="name" {
                    "Dein Name"
                }
                input.bamboo-input #name required type="text" name="name" value=(name) ;
                label.bamboo-label for="email" {
                    "Deine Email"
                }
                input.bamboo-input #email required type="email" name="email" value=(email) ;
                button.bamboo-button type="submit" {
                    "Registrieren"
                }
            }
        },
    )
}

#[get("/register")]
pub async fn register_page() -> impl Responder {
    register_form(false, "", "")
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct RegisterForm {
    email: String,
    name: String,
}

#[post("/register")]
pub async fn send_registration(body: web::Form<RegisterForm>, db: DbConnection) -> impl Responder {
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum Response {
        Sent,
        Exists,
    }

    let email = body.email.clone();
    let name = body.name.clone();

    let response = if dbal::user_exists(&email, &db).await {
        Response::Exists
    } else {
        bamboo_common::backend::mailing::enqueue_register_mail(&name, &email, &db).await;
        Response::Sent
    };

    if response == Response::Exists {
        register_form(true, &name, &email)
    } else {
        banner_page(
            "Werde ein Panda",
            html! {
                h1 {
                    "Deine Email ist unterwegs"
                }
                p {
                    "Klasse, es hat alles geklappt und deine Registrierungsemail ist unterwegs zu dir. Klick einfach auf den Button in der Email und du kannst direkt deinen Account anlegen."
                }
            },
        )
    }
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct CreateAccountQuery {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct CreateAccountForm {
    name: String,
    email: String,
    discord: Option<String>,
    password: String,
}

fn create_account_form(error: bool, name: &str, email: &str) -> Markup {
    banner_page(
        "Werde ein Panda",
        html! {
            h1 {
                "Erstell deinen Account"
            }
            p {
                "Du hast dich entschiedenen einen Account zu erstellen, großartig. Du musst unten nur das Formular ausfüllen und abschicken. Anschließend kannst du direkt loslegen."
            }
            @if error {
                p.bamboo-error {
                    "Leider konnten wir deinen Account nicht erstellen. Wende dich doch bitte an den Bambussupport unter "
                    a href="mailto:panda.helferlein@bambushain.app" { "panda.helferlein@bambushain.app" }
                    ", wir werden unser bestes tun dir zu helfen."
                }
            }
            form.bamboo-form action="/create-account" method="post" {
                input type="hidden" name="name" value=(name) ;
                input type="hidden" name="email" value=(email) ;
                label.bamboo-label for="name" {
                    "Dein Name"
                }
                input.bamboo-input #name required type="text" readonly value=(name) ;
                label.bamboo-label for="email" {
                    "Deine Email"
                }
                input.bamboo-input #email required type="email" readonly value=(email) ;
                label.bamboo-label for="discord_name" {
                    "Dein Discord Name (optional)"
                }
                input.bamboo-input #discord_name type="text" name="discord_name" ;
                label.bamboo-label for="password" {
                    "Dein Passwort"
                }
                input.bamboo-input #password minlength=8 required type="password" name="password" ;
                button.bamboo-button type="submit" {
                    "Account erstellen"
                }
            }
        },
    )
}

#[get("/create-account")]
pub async fn create_account_page(query: web::Query<CreateAccountQuery>) -> impl Responder {
    let name = base64::engine::general_purpose::URL_SAFE
        .decode(&query.name)
        .map(|v| String::from_utf8(v));
    let email = base64::engine::general_purpose::URL_SAFE
        .decode(&query.email)
        .map(|v| String::from_utf8(v));

    if let (Ok(Ok(name)), Ok(Ok(email))) = (name, email) {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(false, &name, &email))
    } else {
        HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()
    }
}

#[post("/create-account")]
pub async fn create_account(
    body: web::Form<CreateAccountForm>,
    db: DbConnection,
) -> impl Responder {
    if let Err(err) = dbal::create_user(
        User::new(
            body.email.clone(),
            body.name.clone(),
            body.discord.clone().unwrap_or_default(),
        ),
        &body.password,
        &db,
    )
    .await
    {
        log::error!("Failed to create account for new user: {err}");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(true, &body.name, &body.email))
    } else {
        let token = if let Ok(token) = dbal::create_token(&body.email, &db).await.map_err(|err| {
            log::error!("Failed to login {err}");
            BambooError::unauthorized("user", "Login data is invalid")
        }) {
            token.token
        } else {
            "".to_string()
        };

        let cookie = Cookie::build(cookie::BAMBOO_AUTH_COOKIE, token)
            .path("/")
            .http_only(true)
            .finish();

        HttpResponse::Found()
            .append_header(("Location", "/authentication/login"))
            .cookie(cookie)
            .finish()
    }
}
