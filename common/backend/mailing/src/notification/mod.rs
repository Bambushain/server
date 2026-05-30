use crate::enqueue_mail;
use bamboo_common_backend_dbal as dbal;
use bamboo_common_core::entities::{Grove, GroveEvent, Mail};
use bamboo_common_core::queueing::notifications::Notification;
use chrono::Locale;
use maud::html;
use sea_orm::DatabaseConnection;

fn event_reminder(event: &GroveEvent, display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("{} findet bald statt", event.title),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "Denk dran, am "
                    @if let Some(start_time) = event.start_time {
                        (event.start_date.and_time(start_time).and_utc().format_localized("%A den %-d. %B %Y um %R", Locale::de_DE_euro))
                    } @else {
                        (event.start_date.format_localized("%A den %-d. %B %Y", Locale::de_DE_euro))
                    } " findet das Ereignis " (event.title) " statt."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }.into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn grove_join(grove: &Grove, joinee: &str, display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("{joinee} ist {} beigetreten", grove.name),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    (joinee) " ist deinem Hain " (grove.name) " beigetreten."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
        .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn grove_ban(grove: &Grove, banee: &str, display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("{banee} wurde aus {} gebannt", grove.name),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    (banee) " wurde aus deinem Hain " (grove.name) " gebannt."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
        .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn grove_unban(grove: &Grove, unbanee: &str, display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("Ban von {unbanee} wurde aufgehoben"),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "der Ban von " (unbanee) " für deinem Hain " (grove.name) " wurde aufgehoben."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
        .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn grove_mods(grove: &Grove, mods: &[&str], display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("Mods von {} wurden geändert", grove.name),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "die Mods in deinem Hain " (grove.name) " wurden geändert." br;
                    "Die neuen Mods sind:" br;
                    ul {
                        @for m in mods {
                            li {
                                (m)
                            }
                        }
                    }
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
        .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn grove_delete(grove: &Grove, display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        format!("Hain {} wurde gelöscht", grove.name),
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "der Hain " (grove.name) " wurde gelöscht. Alle Kalendereinträge wurden ebenfalls entfernt."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
            .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn user_password_changed(display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        "Dein Passwort wurde geändert",
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "dein Passwort wurde erfolgreich geändert. Bitte denk daran, dass du deine Zwei Faktor Authentifizierung neu einrichten musst."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
            .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

fn user_account_delete(display_name: &str, email: &str) -> Mail {
    Mail::new_templated(
        "Dein Account wurde gelöscht",
        email,
        html! {
            mj-text {
                p {
                    "Hey " (display_name) ","
                }
                p {
                    "dein Account wurde erfolgreich gelöscht, schade dass du gehst. Ich hoffe wir können dich in der Zukunft zurück gewinnen."
                }
                p {
                    "Alles Gute" br;
                    "Dein Panda Helferlein"
                }
            }
        }
            .into_string(),
        None as Option<String>,
        None as Option<String>,
        None as Option<String>,
    )
}

pub async fn enqueue_notification(notification: &Notification, db: &DatabaseConnection) {
    match notification {
        Notification::EventReminder(event, _) => {
            if event.is_private
                && let Some(ref user) = event.user
            {
                enqueue_mail(event_reminder(event, &user.display_name, &user.email), db).await;
            } else if !event.is_private
                && let Some(ref grove) = event.grove
                && let Ok(users) = dbal::get_all_users_by_grove(grove.id, db).await
            {
                for user in users {
                    enqueue_mail(event_reminder(event, &user.display_name, &user.email), db).await;
                }
            }
        }
        Notification::GroveJoin(grove, user) => {
            if let Ok(mods) = dbal::get_grove_mods(grove.id, db).await {
                for r#mod in mods {
                    enqueue_mail(
                        grove_join(grove, &user.display_name, &r#mod.display_name, &r#mod.email),
                        db,
                    )
                    .await;
                }
            }
        }
        Notification::GroveBan(grove, user) => {
            if let Ok(mods) = dbal::get_grove_mods(grove.id, db).await {
                for r#mod in mods {
                    enqueue_mail(
                        grove_ban(grove, &user.display_name, &r#mod.display_name, &r#mod.email),
                        db,
                    )
                    .await;
                }
            }
        }
        Notification::GroveUnban(grove, user) => {
            if let Ok(mods) = dbal::get_grove_mods(grove.id, db).await {
                for r#mod in mods {
                    enqueue_mail(
                        grove_unban(grove, &user.display_name, &r#mod.display_name, &r#mod.email),
                        db,
                    )
                    .await;
                }
            }
        }
        Notification::GroveModChange(grove) => {
            if let Ok(ref mods) = dbal::get_grove_mods(grove.id, db).await {
                let current_mods = mods
                    .iter()
                    .map(|m| m.display_name.as_str())
                    .collect::<Vec<_>>();
                for r#mod in mods {
                    enqueue_mail(
                        grove_mods(grove, &current_mods, &r#mod.display_name, &r#mod.email),
                        db,
                    )
                    .await;
                }
            }
        }
        Notification::GroveDelete(grove, users) => {
            for user in users {
                enqueue_mail(grove_delete(grove, &user.display_name, &user.email), db).await;
            }
        }
        Notification::UserPasswordChange(user) => {
            enqueue_mail(user_password_changed(&user.display_name, &user.email), db).await;
        }
        Notification::UserAccountDelete(user) => {
            enqueue_mail(user_account_delete(&user.display_name, &user.email), db).await;
        }
        _ => {}
    }
}
