use crate::login::{ForgotPasswordAction, LoginAction, ResetPasswordAction};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_window;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/authentication/pkg/frontend-authentication.css"/>
        <Title formatter=|text| format!("{text} – Bambushain") />
        <Link href="/authentication/assets/favicon.svg" rel="icon" type_="image/svg+xml" />
        <Link href="/authentication/assets/favicon.png" rel="icon" type_="image/png" />

        <Link href="/authentication/assets/manifest.json" rel="manifest" />
        <Link href="/authentication/assets/favicon.svg" rel="mask-icon" />

        <Meta content="#598c79" name="msapplication-TileColor" />
        <Meta content="#598c79" name="theme-color" />
        <Meta content="width=device-width, initial-scale=1" name="viewport" />

        <Router>
            <Routes>
                <Route path="/authentication" view=Login/>
                <Route path="/authentication/forgot-password" view=ForgotPassword/>
                <Route path="/authentication/reset-password" view=ResetPassword/>
                <Route path="/authentication/*" view=Login/>
            </Routes>
        </Router>
    }
}

#[component]
fn Login() -> impl IntoView {
    let login = create_server_action::<LoginAction>();
    let value = login.value();

    let two_factor_visible = move || match value.get() {
        Some(Ok(value)) => value.requires_two_factor,
        Some(Err(_)) => false,
        None => false,
    };
    let has_error = move || match value.get() {
        Some(Ok(value)) => !value.login_success,
        Some(Err(_)) => true,
        None => false,
    };

    create_effect(move |_| {
        if let Some(Ok(value)) = value.get() {
            if value.login_success {
                let window = use_window();
                let _ = window.as_ref().unwrap().location().set_href("/pandas");
            }
        }
    });

    view! {
        <Title text="Anmelden" />
        <div class="auth-container">
            <div class="auth-box">
                <h1>Anmelden</h1>
                <p class="login-message">
                    <Show when=has_error fallback=|| "Melde dich an und betrete den Bambushain">
                        Zu den Anmeldedaten wurde kein Benutzer gefunden
                    </Show>
                </p>
                <ActionForm action=login class="auth-form">
                    <div class="auth-fields">
                        <label>Email oder Name</label>
                        <input type="text" name="email" required />
                        <label>Passwort</label>
                        <input type="password" name="password" required />
                        <Show when=two_factor_visible>
                            <label>Zwei Faktor Code</label>
                            <input type="text" maxlength="6" name="two_factor_code" required />
                        </Show>
                        <div class="auth-buttons">
                            <a href="/authentication/forgot-password" class="auth-button">Passwort vergessen</a>
                            <button type="submit" class="auth-button">Anmelden</button>
                        </div>
                    </div>
                </ActionForm>
                <footer class="auth-footer">
                    <a href="/legal/imprint">Impressum</a>
                    <a href="/legal/data-protection">Datenschutz</a>
                </footer>
            </div>
        </div>
    }
}

#[component]
fn ForgotPassword() -> impl IntoView {
    let forgot = create_server_action::<ForgotPasswordAction>();
    let value = forgot.value();

    let sent = move || value.get().is_some();

    view! {
        <Title text="Passwort vergessen" />
        <div class="auth-container">
            <div class="auth-box">
                <h1>Passwort vergessen</h1>
                <p class="login-message">
                    <Show when=sent fallback=|| "Gib unten deine Email oder Benutzernamen ein und dir wird ein Link zugeschickt">
                        Wenn wir zu deinen Daten einen Benutzer haben, schicken wir dir einen Link zu
                    </Show>
                </p>
                <ActionForm action=forgot class="auth-form">
                    <div class="auth-fields">
                        <label>Email oder Name</label>
                        <input type="text" name="email" required />
                        <div class="auth-buttons is--reset">
                            <button type="submit" class="auth-button">Link zuschicken</button>
                        </div>
                    </div>
                </ActionForm>
                <footer class="auth-footer">
                    <a href="/legal/imprint">Impressum</a>
                    <a href="/legal/data-protection">Datenschutz</a>
                </footer>
            </div>
        </div>
    }
}

#[derive(Params, PartialEq, Clone)]
struct ResetPasswordQuery {
    pub token: Option<String>,
    pub email: Option<String>,
}

#[component]
fn ResetPassword() -> impl IntoView {
    let reset = create_server_action::<ResetPasswordAction>();
    let value = reset.value();
    let (password, set_password) = create_signal("".to_string());
    let (password_repeat, set_password_repeat) = create_signal("".to_string());

    let query = use_query::<ResetPasswordQuery>();
    let token = move || query.get().map(|res| res.token).unwrap_or(Some("".into()));
    let email = move || query.get().map(|res| res.email).unwrap_or(Some("".into()));

    let has_error = move || match value.get() {
        Some(Ok(value)) => !value,
        Some(Err(_)) => true,
        None => false,
    };
    let reset_enabled = move || password.get().eq(&password_repeat.get());

    create_effect(move |_| {
        if let Some(Ok(value)) = value.get() {
            if value {
                let window = use_window();
                let _ = window
                    .as_ref()
                    .unwrap()
                    .location()
                    .set_href("/authentication");
            }
        }
    });

    view! {
        <Title text="Passwort zurücksetzen" />
        <div class="auth-container">
            <div class="auth-box">
                <h1>Passwort zurücksetzen</h1>
                <p class="login-message">
                    <Show when=has_error fallback=|| "Gib unten dein neues Passwort ein und bestätige es">
                        <span>
                            "Leider konnte dein Passwort nicht geändert werden, bitte wende dich an den "<a href="mailto:panda.helferlein@bambushain.app">Support</a>
                        </span>
                    </Show>
                </p>
                <ActionForm action=reset class="auth-form">
                    <div class="auth-fields">
                        <input type="hidden" name="token" prop:value=token />
                        <input type="hidden" name="email" prop:value=email />
                        <label>Neues Passwort</label>
                        <input type="password" name="password" prop:value=password on:input=move |ev| set_password.set(event_target_value(&ev)) required />
                        <label>Neues Passwort wiederholen</label>
                        <input type="password" prop:value=password_repeat on:input=move |ev| set_password_repeat.set(event_target_value(&ev)) required />
                        <div class="auth-buttons">
                            <a href="/authentication" class="auth-button">Zum Login</a>
                            <button type="submit" class="auth-button" disabled=move || !reset_enabled()>Passwort setzen</button>
                        </div>
                    </div>
                </ActionForm>
                <footer class="auth-footer">
                    <a href="/legal/imprint">Impressum</a>
                    <a href="/legal/data-protection">Datenschutz</a>
                </footer>
            </div>
        </div>
    }
}
