use crate::enqueue_mail;
use bamboo_common_backend_dbal as dbal;
use bamboo_common_core::queueing::Mail;
use chrono::Locale;
use maud::html;
use sea_orm::DatabaseConnection;

pub async fn enqueue_forgot_password_mail(email: String, db: &DatabaseConnection) {
    if let Ok(user) = dbal::get_user_by_email_or_username(email, db).await {
        if let Ok((token, valid_until)) = dbal::set_forgot_password_token(user.id, db).await {
            let mail_body = html! {
                mj-text {
                    p {
                        (format!("Hey {},", user.display_name))
                    }
                    p {
                        "du willst dein Passwort zurücksetzen?" br;
                        "Falls ja, klick einfach unten auf den Button du kannst dann ein neues Passwort vergeben." br;
                        (format!("Der Link ist bis {} gültig.", valid_until.format_localized("%A den %-d. %B %C%y", Locale::de_DE_euro)))
                    }
                    p {
                       "Bitte beachte, dass deine Zwei Faktor Authentifizierung zurückgesetzt wird."
                    }
                    p {
                        "Alles Gute" br;
                        "Dein Panda Helferlein"
                    }
                }
            }.into_string();

            enqueue_mail(Mail::new_templated(
                "Passwort vergessen",
                user.email.clone(),
                mail_body,
                None as Option<String>,
                "Passwort zurücksetzen",
                format!(
                    "https://bambushain.app/authentication/reset-password?token={token}&email={}",
                    user.email
                ),
            ))
            .await;
        }
    }
}
