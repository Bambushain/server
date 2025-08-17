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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum RegisterError {
    InvalidEmail,
    InvalidName,
    UserExists,
}

fn register_form(error: Option<RegisterError>, name: &str, email: &str) -> Markup {
    banner_page(
        "Werde ein Panda",
        html! {
            h1 {
                "Registriere dich für Bambushain"
            }
            p {
                "Hier kannst du dich für Bambushain registrieren. Gib einfach deine Emailadresse ein und du bekommst einen Link zum Registrieren."
            }
            @if error.is_some() {
                @match error {
                    Some(RegisterError::InvalidEmail) => {
                        p.bamboo-error {
                            "Deine Emailadresse ist ungültig, bitte schau nochmal nach ob du die korrekte Adresse eingegeben hast."
                        }
                    }
                    Some(RegisterError::InvalidName) => {
                        p.bamboo-error {
                            "Der von dir gewählte Name ist zu kurz. Er muss mindestens 3 Zeichen lang sein."
                        }
                    }
                    Some(RegisterError::UserExists) => {
                        p.bamboo-info {
                            "So wie es aussieht hast du schon einen Account. Wenn du dich unter diesem Account anmelden möchtest, klick einfach "
                            a href="/authentication/login" { "hier" }
                            " und du kannst dich direkt einloggen oder dein Passwort zurücksetzen."
                        }
                    }
                    _ => {}
                }
            }
            form.bamboo-form action="/register" method="post" {
                label.bamboo-label for="name" {
                    "Dein Name"
                }
                input.bamboo-input #name required minlength="3" type="text" name="name" value=(name) ;
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
    register_form(None, "", "")
}

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct RegisterForm {
    email: String,
    name: String,
}

#[post("/register")]
pub async fn send_registration(body: web::Form<RegisterForm>, db: DbConnection) -> impl Responder {
    let email = body.email.clone();
    let name = body.name.clone();

    if !email_address::EmailAddress::is_valid(&email) {
        register_form(Some(RegisterError::InvalidEmail), &name, &email)
    } else if name.len() < 3 {
        register_form(Some(RegisterError::InvalidName), &name, &email)
    } else if dbal::user_exists(&email, &db).await {
        register_form(Some(RegisterError::UserExists), &name, &email)
    } else {
        bamboo_common::backend::mailing::enqueue_register_mail(&name, &email, &db).await;
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
enum CreateAccountError {
    InvalidEmail,
    InvalidPassword,
    UserExists,
    Unknown,
}

fn create_account_form(
    error: Option<CreateAccountError>,
    name: &str,
    email: &str,
    discord: Option<String>,
) -> Markup {
    banner_page(
        "Werde ein Panda",
        html! {
            h1 {
                "Erstell deinen Account"
            }
            p {
                "Du hast dich entschiedenen einen Account zu erstellen, großartig. Du musst unten nur das Formular ausfüllen und abschicken. Anschließend kannst du direkt loslegen."
            }
            @if error.is_some() {
                p.bamboo-error {
                    @match error {
                        Some(CreateAccountError::InvalidEmail) => {
                            "Deine Emailadresse ist ungültig, bitte schau nochmal nach ob du die korrekte Adresse eingegeben hast."
                        }
                        Some(CreateAccountError::InvalidPassword) => {
                            "Dein Passwort ist zu kurz. Es muss mindestens 8 Zeichen lang sein."
                        }
                        Some(CreateAccountError::UserExists) => {
                            "Ein Account mit dieser Email existiert bereits. Du kannst dich "
                            a href="/pandas" { "hier" }
                            " anmelden oder dein Passwort zurücksetzen."
                        }
                        Some(CreateAccountError::Unknown) => {
                            "Leider konnten wir deinen Account nicht erstellen. Wende dich doch bitte an den Bambussupport unter "
                            a href="mailto:panda.helferlein@bambushain.app" { "panda.helferlein@bambushain.app" }
                            ", wir werden unser bestes tun dir zu helfen."
                        }
                        _ => {}
                    }
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
                input.bamboo-input #discord_name type="text" name="discord_name" value=(discord.unwrap_or_default()) ;
                label.bamboo-label for="password" {
                    "Dein Passwort"
                }
                input.bamboo-input #password minlength="8" required type="password" name="password" ;
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
        .map(String::from_utf8);
    let email = base64::engine::general_purpose::URL_SAFE
        .decode(&query.email)
        .map(String::from_utf8);

    if let (Ok(Ok(name)), Ok(Ok(email))) = (name, email) {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(None, &name, &email, None))
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
    let email = body.email.clone();
    let name = body.name.clone();
    let password = body.password.clone();

    if !email_address::EmailAddress::is_valid(&email) {
        log::error!("Invalid email address: {email}");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(
                Some(CreateAccountError::InvalidEmail),
                &body.name,
                &body.email,
                body.discord.clone(),
            ))
    } else if password.len() < 8 {
        log::error!("Invalid password");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(
                Some(CreateAccountError::InvalidPassword),
                &body.name,
                &body.email,
                body.discord.clone(),
            ))
    } else if dbal::user_exists(&email, &db).await {
        log::error!("User already exists: {email}");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(
                Some(CreateAccountError::UserExists),
                &body.name,
                &body.email,
                body.discord.clone(),
            ))
    } else if let Err(err) = dbal::create_user(
        User::new(email, name, body.discord.clone().unwrap_or_default()),
        &body.password,
        &db,
    )
    .await
    {
        log::error!("Failed to create account for new user: {err}");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(create_account_form(
                Some(CreateAccountError::Unknown),
                &body.name,
                &body.email,
                body.discord.clone(),
            ))
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
