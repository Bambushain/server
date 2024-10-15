use crate::api::{get_all_groves, CreateGroveAction};
use crate::state::AllGroves;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn NewGrovePage() -> impl IntoView {
    let name = create_rw_signal(String::new());

    let invite_on = create_rw_signal(true);

    let create_grove_action = create_server_action::<CreateGroveAction>();

    let groves_resource = create_local_resource(|| {}, |_| async move { get_all_groves().await });
    let groves_context = expect_context::<RwSignal<AllGroves>>();

    let navigate = use_navigate();

    create_effect(move |_| {
        if let Some(Ok(groves)) = groves_resource.get() {
            groves_context.set(groves);
        }
    });
    create_effect(move |_| {
        if let Some(Ok(grove)) = create_grove_action.value().get() {
            let navigate = navigate.clone();
            groves_resource.refetch();

            navigate(
                format!("/pandas/groves/{}/{}", grove.id, grove.name).as_str(),
                NavigateOptions::default(),
            )
        }
    });

    view! {
        <div class="pandas-grove__new">
            <leptos_meta::Title text="Neuer Hain" />
            <Title title="Neuer Hain" />
            <p>
                {"Cool, dass du deinen eigenen Hain erstellen möchtest. Dafür brauchen wir zwei kleine Infos von dir, einmal einen Namen und die Bestätigung, dass andere Pandas in den Hain eingeladen werden können. Füll das Formular unten einfach aus, klick auf Hain erstellen und schon bist du fertig."}
            </p>
            <Show when=move || create_grove_action.value().get().is_some_and(|res| res.is_err())>
                <AlertMessage header="Fehler beim Erstellen" message_type=MessageType::Negative>
                    <MessageContent slot>
                        <p>
                            {"Tut uns leid, der Hain konnte leider nicht erstellt werden. Bitte wende dich an den Bambussupport"}
                        </p>
                    </MessageContent>
                </AlertMessage>
            </Show>
            <ActionForm
                buttons=Box::new(|| {
                    view! { <Button is_submit=true label="Hain erstellen" /> }.into()
                })
                action=create_grove_action
            >
                <Textbox name="name" label="Name" value=name />
                <Switch name="allow_invite" label="Einladungen aktiv" checked=invite_on />
            </ActionForm>
        </div>
    }
}
