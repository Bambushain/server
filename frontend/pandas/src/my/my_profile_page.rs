use crate::api::{
    get_all_groves, get_profile, get_qr_code, update_profile_picture, ChangePasswordAction,
    DeleteProfileAction, DisableTotpAction, PasswordResponse, UpdateProfileAction,
    ValidateTotpAction,
};
use bamboo_common::core::entities::User;
use gloo_file::futures::read_as_bytes;
use leptos::prelude::{ActionForm, *};
use leptos::task::spawn_local;
use leptos_cosmo::prelude::*;
use leptos_meta as meta;
use leptos_router::components::A;
use leptos_use::use_window;

#[component]
fn UpdateProfileDialog(#[prop(into)] on_close: Callback<(), ()>) -> impl IntoView {
    let update_profile_action = ServerAction::<UpdateProfileAction>::new();

    let current_user_ctx = expect_context::<RwSignal<User>>();

    let email = RwSignal::new(current_user_ctx.get().email.clone());
    let name = RwSignal::new(current_user_ctx.get().display_name.clone());
    let discord_name = RwSignal::new(current_user_ctx.get().discord_name.clone());
    let profile_picture = RwSignal::new_local(None);
    let profile_picture_to_set = profile_picture.clone();

    let has_error = RwSignal::new(false);
    let error_message = RwSignal::new(String::new());
    let error_message_header = RwSignal::new(String::new());

    Effect::new(move |_| {
        if let Some(Ok(res)) = update_profile_action.value().get() {
            if res.success && res.user.is_some() {
                current_user_ctx.set(res.user.unwrap());
                if let Some(profile_picture) = profile_picture.get() {
                    let files = gloo_file::FileList::from(profile_picture);
                    if let Some(file) = files.iter().next().cloned() {
                        spawn_local(async move {
                            if let Ok(data) = read_as_bytes(&file).await {
                                if update_profile_picture(data).await.is_ok() {
                                    on_close.run(())
                                } else {
                                    has_error.set(true);
                                    error_message.set("Das Profilbild konnte leider nicht hochgeladen werden, bitte wende dich an den Bambussupport".into());
                                    error_message_header.set("Fehler beim Hochladen".into());
                                }
                            } else {
                                on_close.run(())
                            }
                        });
                    } else {
                        on_close.run(())
                    }
                } else {
                    on_close.run(())
                }
            } else {
                has_error.set(true);
                error_message.set(res.message);
                error_message_header.set(res.header);
            }
        }
    });

    view! {
        <ActionFormModal
            title="Profil bearbeiten"
            action=update_profile_action
            has_error=has_error
            error_message=error_message
            error_message_header=error_message_header
        >
            <ModalContent slot>
                <Textbox
                    label="Email"
                    name="email"
                    input_type=TextBoxType::Email
                    required=true
                    value=email
                />
                <Textbox label="Name" name="display_name" required=true value=name />
                <Textbox
                    label="Discord Name (optional)"
                    name="discord_name"
                    value=discord_name
                    required=false
                />
                <FilePicker
                    label="Profilbild (optional)"
                    file_picked=move |file| profile_picture_to_set.set(Some(file))
                />
            </ModalContent>
            <ModalButton slot on_click=move || on_close.run(()) label="Schließen" />
            <ModalButton slot is_submit=true label="Profil speichern" />
        </ActionFormModal>
    }
}

#[component]
fn EnableTotpDialog(#[prop(into)] on_close: Callback<(), ()>) -> impl IntoView {
    let qr_code_resource = Resource::new(|| {}, |_| async move { get_qr_code().await });

    let validate_totp_action = ServerAction::<ValidateTotpAction>::new();

    let code_state = RwSignal::new(String::new());
    let password_state = RwSignal::new(String::new());

    let error_message = RwSignal::new(String::new());
    let error_message_header = RwSignal::new(String::new());
    let has_error = RwSignal::new(false);

    Effect::new(move |_| {
        if validate_totp_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok_and(|res| res))
        {
            let window = use_window();
            let _ = window
                .as_ref()
                .unwrap()
                .location()
                .set_href("/authentication");
        } else if validate_totp_action
            .value()
            .get()
            .is_some_and(|res| res.is_err())
        {
            error_message.set("Zwei Faktor per App konnte leider nicht aktiviert werden. Bitte wende dich an den Bambussupport.".into());
            error_message_header.set("Fehler beim Aktivieren".into());
            has_error.set(true);
        } else if validate_totp_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok_and(|res| !res))
        {
            error_message.set(
                "Der von dir eingegebene Code oder dein Passwort ist ungültig, versuch es nochmal."
                    .into(),
            );
            error_message_header.set("Code oder Passwort falsch".into());
            has_error.set(true);
        }
    });

    view! {
        <ActionForm action=validate_totp_action>
            <Modal title="Zwei Faktor per App aktivieren">
                <ModalContent slot>
                    <div class="pandas-totp__container">
                        <Suspense fallback=|| {
                            view! { <ProgressRing /> }
                        }>
                            {move || Suspend::new(async move {
                                qr_code_resource
                                    .await
                                    .map(|qr_code| {
                                        view! {
                                            <img
                                                class="pandas-totp__code"
                                                src=qr_code.qr_code.clone()
                                                alt=qr_code.secret.clone()
                                            />
                                            <svg class="pandas-totp__logo" viewBox="0 0 512 512">
                                                <path d="M511.094,264.722c-1.136-3.307-28.511-81.137-89.171-95.166c-30.729-7.107-63.124,3.303-96.526,30.938v-35.663
                                                c6.222-2.428,10.637-8.464,10.637-15.545s-4.415-13.117-10.637-15.545V21.124c0-9.22-7.475-16.696-16.696-16.696h-89.595
                                                c-9.22,0-16.696,7.475-16.696,16.696v46.166c-18.137-33.54-41.579-53.478-69.951-59.406C71.508-4.849,13.992,54.3,11.574,56.825
                                                C6.875,61.728,5.615,68.989,8.387,75.19c2.773,6.2,9.015,10.103,15.811,9.873c82.495-2.81,169.04,34.422,169.902,34.798
                                                c2.146,0.936,4.415,1.391,6.668,1.391c0.55,0,1.097-0.031,1.643-0.085v12.741c-5.986,2.538-10.185,8.467-10.185,15.378
                                                s4.2,12.84,10.185,15.378v99.481c-13.69-36.175-34.515-59.305-62.158-68.907C81.436,174.809,16.819,226.106,14.098,228.3
                                                c-5.288,4.262-7.467,11.302-5.513,17.805c1.956,6.503,7.654,11.176,14.416,11.815c6.876,0.651,13.745,1.588,20.559,2.751
                                                c-26.815,24.958-41.321,57.285-42.141,59.145c-2.739,6.214-1.443,13.469,3.281,18.349c3.208,3.314,7.561,5.083,11.999,5.083
                                                c2.096,0,4.212-0.395,6.233-1.209c76.563-30.832,170.624-25.43,171.564-25.372c2.816,0.178,5.51-0.359,7.913-1.449v27.787
                                                c-5.986,2.538-10.185,8.467-10.185,15.378s4.2,12.84,10.185,15.378v117.115c0,9.22,7.475,16.696,16.696,16.696H308.7
                                                c9.22,0,16.696-7.475,16.696-16.696V373.928c6.222-2.428,10.637-8.464,10.637-15.545s-4.415-13.117-10.637-15.545v-97.236
                                                c22.507,1.287,99.826,7.886,162.387,39.448c2.383,1.202,4.958,1.79,7.516,1.79c3.954,0,7.87-1.404,10.977-4.113
                                                C511.396,278.264,513.3,271.144,511.094,264.722z M70.033,53.522c16.303-9.503,36.4-16.998,55.681-12.936
                                                c16.129,3.398,30.358,14.887,42.528,34.277C142.992,66.766,107.92,57.514,70.033,53.522z M55.265,296.723
                                                c8.409-10.079,18.888-19.87,31.085-25.859c14.339,4.315,27.897,9.235,40.144,14.176
                                                C104.959,286.978,80.307,290.495,55.265,296.723z M72.688,232.553c17.389-7.306,38.216-12.161,56.607-5.773
                                                c15.598,5.418,28.267,18.643,37.87,39.466C143.202,255.001,109.679,241.362,72.688,232.553z M292.005,474.18h-56.204v-99.102
                                                h56.204V474.18z M292.005,341.687h-56.204V165.981h56.204V341.687z M292.005,132.589h-56.204v-94.77h56.204V132.589z
                                                M361.327,215.325c19.184-12.489,36.925-16.945,52.99-13.256c19.207,4.408,34.299,19.645,45.106,35.114
                                                C423.36,224.901,387.642,218.575,361.327,215.325z" />
                                            </svg>
                                        }
                                    })
                            })}
                        </Suspense>
                        <div class="pandas-totp__details">
                            <AlertMessage
                                header="Schritte zum Aktivieren"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    {"Zu erst musst du den QR Code mit einer App wie Authy oder dem Google Authenticator scannen."}
                                    <br />
                                    {"Anschließend gibst du in den Feldern dein aktuelles Passwort ein und der Code der dir in der App angezeigt wird."}
                                </MessageContent>
                            </AlertMessage>
                            <AlertMessage
                                header="Du wirst abgemeldet"
                                message_type=MessageType::Warning
                            >
                                <MessageContent slot>
                                    {"Nach erfolgreicher Aktivierung wirst du abgemeldet."}
                                </MessageContent>
                            </AlertMessage>
                            <AlertMessage
                                header=error_message_header
                                message_type=MessageType::Negative
                                visible=has_error
                            >
                                <MessageContent slot>{error_message}</MessageContent>
                            </AlertMessage>
                            <InputGroup>
                                <Textbox
                                    input_type=TextBoxType::Password
                                    label="Aktuelles Passwort"
                                    required=true
                                    name="password"
                                    value=password_state
                                />
                                <Textbox
                                    label="Zwei Faktor Code"
                                    required=true
                                    value=code_state
                                    name="totp_code"
                                />
                            </InputGroup>
                        </div>
                    </div>
                </ModalContent>
                <ModalButton slot on_click=on_close label="Abbrechen" />
                <ModalButton slot is_submit=true label="App einrichten" />
            </Modal>
        </ActionForm>
    }
}

#[component]
fn ChangePasswordDialog(#[prop(into)] on_close: Callback<(), ()>) -> impl IntoView {
    let old_password = RwSignal::new(String::new());
    let new_password = RwSignal::new(String::new());

    let change_password_action = ServerAction::<ChangePasswordAction>::new();

    let error_message = RwSignal::new(String::new());
    let error_message_header = RwSignal::new(String::new());
    let has_error = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(Ok(res)) = change_password_action.value().get() {
            match res {
                PasswordResponse::WrongPassword => {
                    error_message.set("Falls du dein Passwort vergessen hast, melde dich bitte ab und klicke auf Passwort vergessen".into());
                    error_message_header.set("Das alte Passwort ist falsch".into());
                    has_error.set(true);
                }
                PasswordResponse::UserNotFound => {
                    error_message
                        .set("Bitte versuch es erneut um einen Fehler auszuschließen".into());
                    error_message_header.set("Du wurdest scheinbar gelöscht".into());
                    has_error.set(true);
                }
                PasswordResponse::Success => {
                    has_error.set(false);
                    on_close.run(());

                    let window = use_window();
                    let _ = window
                        .as_ref()
                        .unwrap()
                        .location()
                        .set_href("/authentication");
                }
            }
        } else if change_password_action
            .value()
            .get()
            .is_some_and(|res| res.is_err())
        {
            error_message.set("Leider konnte dein Passwort nicht geändert werden".into());
            error_message_header.set("Fehler beim Ändern".into());
            has_error.set(true);
        }
    });

    view! {
        <ActionFormModal
            action=change_password_action
            title="Passwort ändern"
            has_error
            error_message
            error_message_header
        >
            <ModalContent slot>
                <Textbox
                    input_type=TextBoxType::Password
                    label="Aktuelles Passwort"
                    name="old_password"
                    value=old_password
                    required=true
                />
                <Textbox
                    input_type=TextBoxType::Password
                    label="Neues Passwort"
                    name="new_password"
                    value=new_password
                    required=true
                />
            </ModalContent>
            <ModalButton slot on_click=on_close label="Abbrechen" />
            <ModalButton slot is_submit=true label="Passwort ändern" />
        </ActionFormModal>
    }
}

#[component]
pub fn MyProfilePage() -> impl IntoView {
    let profile_resource = Resource::new(|| (), |_| async move { get_profile().await });
    let groves_resource = Resource::new(|| (), |_| async move { get_all_groves().await });

    let delete_profile_action = ServerAction::<DeleteProfileAction>::new();
    let disable_totp_action = ServerAction::<DisableTotpAction>::new();

    let change_password_open = RwSignal::new(false);
    let edit_profile_open = RwSignal::new(false);
    let enable_totp_open = RwSignal::new(false);
    let time = RwSignal::new(Local::now().timestamp_millis());

    let profile = Memo::new(move |_| profile_resource.get().map(|res| res.ok()));
    let display_name = Memo::new(move |_| {
        profile
            .get()
            .unwrap_or_default()
            .map(|res| res.display_name)
            .unwrap_or_default()
    });
    let email = Memo::new(move |_| {
        profile
            .get()
            .unwrap_or_default()
            .map(|res| res.email)
            .unwrap_or_default()
    });
    let discord_name = Memo::new(move |_| {
        profile
            .get()
            .unwrap_or_default()
            .map(|res| res.discord_name)
            .unwrap_or_default()
    });
    let totp_validated = Memo::new(move |_| {
        profile
            .get()
            .unwrap_or_default()
            .map(|res| res.totp_validated.unwrap_or_default())
            .unwrap_or_default()
    });
    let title = Memo::new(move |_| format!("Hallo {}!", display_name.get()));

    let delete_account = move |_| {
        delete_profile_action.dispatch(DeleteProfileAction {});
    };
    let disable_totp = move |_| {
        disable_totp_action.dispatch(DisableTotpAction {});
    };

    let on_open_change_password = move |_| change_password_open.set(true);
    let on_open_profile_edit = move |_| edit_profile_open.set(true);
    let on_delete_account = move |_| {
        use_modals().confirm(
            "Account löschen",
            "Bist du sicher, dass du deinen Account löschen möchtest? Wenn du deinen Account löscht, werden alle deine Daten gelöscht und können nicht wiederhergestellt werden.",
            Variant::Negative,
            "Account löschen",
            "Account behalten",
            Some(Callback::new(delete_account)),
            None,
        )
    };
    let on_open_enable_totp = move |_| {
        enable_totp_open.set(true);
    };
    let on_open_disable_totp = move |_| {
        use_modals().confirm(
            "App Zwei Faktor deaktivieren",
            "Möchtest du deine Zwei Faktor Authentifizierung per App deaktivieren? Anschließend wirst du abgemeldet.",
            Variant::Negative,
            "Deaktivieren",
            "Nicht deaktivieren",
            Some(Callback::new(disable_totp)),
            None,
        )
    };

    let close_edit_profile = move || {
        edit_profile_open.set(false);
        time.set(Local::now().timestamp_millis());
    };
    let close_totp = move || {
        enable_totp_open.set(false);
    };
    let close_change_password = move || {
        change_password_open.set(false);
    };

    Effect::new(move |_| {
        if delete_profile_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            let window = use_window();
            let _ = window
                .as_ref()
                .unwrap()
                .location()
                .set_href("/authentication");
        }
    });
    Effect::new(move |_| {
        if disable_totp_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            let window = use_window();
            let _ = window
                .as_ref()
                .unwrap()
                .location()
                .set_href("/authentication");
        }
    });

    view! {
        <Transition>
            <meta::Title text=move || format!("Profil von {}", display_name.get()) />
            <Title title=title />
            <div class="pandas-profile">
                <Toolbar>
                    <ToolbarGroup>
                        <Button label="Profil bearbeiten" on:click=on_open_profile_edit />
                    </ToolbarGroup>
                    <ToolbarGroup>
                        <Button label="Passwort ändern" on:click=on_open_change_password />
                        <Show
                            when=move || totp_validated.get()
                            fallback=move || {
                                view! {
                                    <Button
                                        label="App Zwei Faktor aktivieren"
                                        on:click=on_open_enable_totp
                                    />
                                }
                            }
                        >
                            <Button
                                label="App Zwei Faktor deaktivieren"
                                on:click=on_open_disable_totp
                            />
                        </Show>
                    </ToolbarGroup>
                    <ToolbarGroup>
                        <Button label="Account löschen" on:click=on_delete_account />
                    </ToolbarGroup>
                </Toolbar>
                <img
                    class="pandas-profile__picture"
                    src=move || {
                        format!(
                            "/api/user/{}/picture#time={}",
                            profile
                                .get()
                                .map(|res| res.map(|res| res.id))
                                .unwrap_or_default()
                                .unwrap_or_default(),
                            time.get(),
                        )
                    }
                />
                <div class="pandas-profile__email">
                    <h2>Emailadresse</h2>
                    <a href=move || format!("mailto:{}", email.get())>{move || email.get()}</a>
                </div>
                <div class="pandas-profile__discord">
                    <h2>Discordname</h2>
                    <p>
                        <Show
                            when=move || discord_name.get().is_empty()
                            fallback=move || discord_name.get()
                        >
                            {"Kein Name hinterlegt"}
                        </Show>
                    </p>
                </div>
                <div class="pandas-profile__groves">
                    <h2>{"Meine Haine"}</h2>
                    <KeyValueList>
                        {move || {
                            Suspend::new(async move {
                                groves_resource
                                    .await
                                    .map(|groves| {
                                        groves
                                            .iter()
                                            .cloned()
                                            .map(|grove| {
                                                let name = grove.name.clone();
                                                view! {
                                                    <dt>{name.clone()}</dt>
                                                    <dd>
                                                        <A href=format!(
                                                            "/pandas/groves/{}/{}",
                                                            grove.id,
                                                            name.clone(),
                                                        )>{format!("{} betreten", name.clone())}</A>
                                                    </dd>
                                                }
                                            })
                                            .collect_view()
                                    })
                            })
                        }}
                    </KeyValueList>
                </div>
                <Show when=move || edit_profile_open.get()>
                    <UpdateProfileDialog on_close=close_edit_profile />
                </Show>
                <Show when=move || enable_totp_open.get()>
                    <EnableTotpDialog on_close=close_totp />
                </Show>
                <Show when=move || change_password_open.get()>
                    <ChangePasswordDialog on_close=close_change_password />
                </Show>
            </div>
        </Transition>
    }
}
