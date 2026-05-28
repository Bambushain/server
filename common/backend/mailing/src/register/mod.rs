use crate::enqueue_mail;
use bamboo_common_core::entities::Mail;
use base64::Engine;
use maud::html;
use sea_orm::DatabaseConnection;

pub async fn enqueue_register_mail(
    name: impl Into<String>,
    email: impl Into<String>,
    db: &DatabaseConnection,
) {
    let email = email.into();
    let name = name.into();
    let base64_name = base64::engine::general_purpose::URL_SAFE.encode(&name);
    let base64_email = base64::engine::general_purpose::URL_SAFE.encode(&email);

    let mail_body = html! {
        mj-text {
            p {
                (format!("Hey {name},"))
            }
            p {
                "schön dass du ein Panda werden willst." br;
                "Ein Panda zu werden geht ganz einfach, klick den Button unten an und du wirst auf die richtige Seite weitergeleitet." br;
            }
            p {
               "Nachdem du deinen Account erstellt hast, kannst du dich im Bambushain anmelden und loslegen."
            }
            p {
                "Alles Gute" br;
                "Dein Panda Helferlein"
            }
        }
    }.into_string();

    enqueue_mail(
        Mail::new_templated(
            "Werde ein Panda im Bambushain",
            &email,
            mail_body,
            None as Option<String>,
            "Erstell deinen Account",
            format!(
                "https://bambushain.app/create-account?name={base64_name}&email={base64_email}"
            ),
        ),
        db,
    )
        .await;
}
