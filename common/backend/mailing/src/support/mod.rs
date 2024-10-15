use bamboo_common_core::queueing::Mail;
use maud::{html, PreEscaped};

pub async fn send_support_request(message: String, subject: String, email: String) {
    let mail_body = html! {
        html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';" {
            head {}
            body {
                (PreEscaped(message.replace("\r\n", "<br>").replace('\n', "<br>")))
            }
        }
    }.into_string();

    crate::enqueue_mail(Mail::new(
        subject,
        "panda.helferlein@bambushain.app",
        mail_body,
        Some(email),
    ))
    .await
}
