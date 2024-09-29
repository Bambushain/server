use crate::api::{
    get_all_groves, get_banned_pandas, get_grove, get_pandas, DeleteGroveAction,
    DisableInvitesAction, EnableInvitesAction, UnbanPandaAction, UpdateModsAction,
};
use crate::state::AllGroves;
use bamboo_common::core::entities::user::GroveUser;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn GroveAdminTab(
    #[prop(into)] grove_id: MaybeSignal<i32>,
    #[prop(into)] grove_name: MaybeSignal<String>,
) -> impl IntoView {
    let navigate = use_navigate();
    let grove_resource = create_local_resource(
        move || grove_id.get(),
        move |id| async move { get_grove(id).await },
    );
    let groves_resource =
        create_local_resource(move || {}, move |_| async move { get_all_groves().await });
    let pandas_resource = create_local_resource_with_initial_value(
        move || grove_id.get(),
        move |id| async move { get_pandas(Some(id)).await },
        Some(Ok(Vec::<GroveUser>::new())),
    );
    let banned_pandas_resource = create_resource_with_initial_value(
        move || grove_id.get(),
        move |id| async move { get_banned_pandas(id).await },
        None,
    );

    let enable_invites_action = create_server_action::<EnableInvitesAction>();
    let disable_invites_action = create_server_action::<DisableInvitesAction>();
    let delete_grove_action = create_server_action::<DeleteGroveAction>();
    let update_mods_action = create_server_action::<UpdateModsAction>();
    let unban_panda_action = create_server_action::<UnbanPandaAction>();

    let selected_mods = create_rw_signal(vec![]);
    let pandas = create_rw_signal(vec![]);

    let groves_ctx = expect_context::<RwSignal<AllGroves>>();

    create_effect(move |_| {
        if let Some(Ok(_)) = enable_invites_action.value().get() {
            grove_resource.refetch()
        }
    });
    {
        let navigate = navigate.clone();

        create_effect(move |_| {
            if grove_resource.get().is_some_and(|res| res.is_err()) {
                navigate("/pandas/groves", NavigateOptions::default());
            }
        });
    }
    create_effect(move |_| {
        if let Some(Ok(_)) = disable_invites_action.value().get() {
            grove_resource.refetch()
        }
    });
    create_effect(move |_| {
        if let Some(Ok(groves)) = groves_resource.get() {
            groves_ctx.set(groves);
        }
    });
    create_effect(move |_| {
        if let Some(Ok(pandas_in_grove)) = pandas_resource.get() {
            pandas.set(
                pandas_in_grove
                    .iter()
                    .map(|panda| (panda.id.to_string(), panda.display_name.clone()))
                    .collect::<Vec<_>>(),
            );
            selected_mods.set(
                pandas_in_grove
                    .iter()
                    .filter_map(|panda| panda.is_mod.then_some(panda.id.to_string()))
                    .collect::<Vec<_>>(),
            )
        }
    });

    let enable_invite =
        move |_| enable_invites_action.dispatch(EnableInvitesAction { id: grove_id.get() });
    let disable_invite =
        move |_| disable_invites_action.dispatch(DisableInvitesAction { id: grove_id.get() });
    let on_delete = Callback::new(move |_| {
        if let Some(Ok(grove)) = grove_resource.get() {
            confirm(
                "Hain löschen",
                format!("Soll der Hain {} wirklich gelöscht werden? Dies löscht auch den Eventkalender.", grove.name),
                Variant::Negative,
                "Hain löschen",
                "Nicht löschen",
                Some(Callback::new(move |_| {
                    delete_grove_action.dispatch(DeleteGroveAction { id: grove_id.get() })
                })),
                None,
            )
        }
    });
    let on_unban = Callback::new(move |user: GroveUser| {
        confirm(
            format!("Ban für {} aufheben", user.display_name),
            format!(
                "Soll der Ban von Panda {} wirklich aufgehoben werden?",
                user.display_name
            ),
            Variant::Warning,
            "Ban aufheben",
            "Gebannt lassen",
            Some(Callback::new(move |_| {
                unban_panda_action.dispatch(UnbanPandaAction {
                    grove_id: grove_id.get(),
                    user_id: user.id,
                })
            })),
            None,
        )
    });

    {
        let navigate = navigate.clone();

        create_effect(move |_| {
            if delete_grove_action
                .value()
                .get()
                .is_some_and(|res| res.is_ok())
            {
                groves_resource.refetch();
                navigate("/pandas/groves", NavigateOptions::default())
            }
        });
    }

    create_effect(move |_| {
        if unban_panda_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            banned_pandas_resource.refetch();
        }
    });

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-grove__management">
                <h1>{format!("Willkommen in der Verwaltung von {}", grove_name.get())}</h1>
                <p>
                    {"Hier hast du die Möglichkeit deinen Hain zu verwalten. Unten findest du den Einladungslink damit Leute deinem Hain beitreten können."}
                    <br />
                    {"Außerdem hast du die Möglichkeit Mods festzulegen. Mods können andere Mods ernennen oder ihnen die Rechte nehmen."}
                    <br /> {"Dazu haben Mods die Rechte Pandas aus einem Hain zu werfen."} <br />
                    {"Daneben können Mods den Hain auch umbennen und vor allem löschen."}
                </p>
                <h2>{"Pandas einladen"}</h2>
                <Show
                    when=move || {
                        if let Some(Ok(grove)) = grove_resource.get() {
                            grove.get_invite_link().is_some()
                        } else {
                            false
                        }
                    }
                    fallback=move || {
                        view! {
                            <p>
                                {"So wie es aussieht hast du Einladungen deaktiviert, wenn du diese aktivieren willst, klick einfach unten auf den Button."}
                                <br />
                                <Button label="Einladungen aktivieren" on:click=enable_invite />
                            </p>
                        }
                    }
                >
                    <p>
                        {"Dein Hain ist ziemlich sinnlos ohne andere Pandas, deswegen kannst du andere Pandas mit dem Link hier direkt in deinen Hain einladen."}
                        <br />
                        {"Einfach kopieren und verschicken, anschließend können andere Pandas deinem Hain beitreten."}
                        <br />
                        <a href=grove_resource
                            .clone()
                            .get()
                            .unwrap()
                            .unwrap()
                            .get_invite_link()
                            .unwrap()>
                            {format!(
                                "https://bambushain.app{}",
                                grove_resource
                                    .clone()
                                    .get()
                                    .unwrap()
                                    .unwrap()
                                    .get_invite_link()
                                    .unwrap(),
                            )}
                        </a>
                    </p>
                    <p>
                        {"Wenn du nicht möchtest, dass weitere Pandas in deinen Hain kommen, kannst du mit einem Klick auf den Button Einladungen deaktivieren."}
                        <br /> <Button label="Einladungen deaktivieren" on:click=disable_invite />
                    </p>
                </Show>
                <h2>{"Modverwaltung"}</h2>
                <p>
                    {"Hier hast du die Möglichkeit die Mods zu verwalten, wähle einfach alle Pandas aus, die du als Mods in deinem Hain neben dir haben willst."}
                </p>
                <ActionForm
                    buttons=Box::new(|| {
                        view! { <Button is_submit=true label="Mods speichern" /> }.into()
                    })
                    action=update_mods_action
                >
                    <MultiSelect label="Mods" items=pandas name="mods" selected=selected_mods />
                </ActionForm>
                <Show when=move || {
                    banned_pandas_resource
                        .get()
                        .is_some_and(|panned_pandas| {
                            panned_pandas.is_ok_and(|banned_pandas| !banned_pandas.is_empty())
                        })
                }>
                    <h3>{"Gebannte Pandas"}</h3>
                    <p>
                        {"So wie es aussieht hattest du schon einmal Probleme mit anderen Pandas in deinem Hain."}
                        <br />
                        {"Das ist echt schade. Aber vielleicht hat sich die Situation schon wieder gebessert, falls ja, kannst du unten den Panda wieder entbannen."}
                    </p>
                    <Table headers=vec![
                        "Name".to_string(),
                        "Email".to_string(),
                        "Discord".to_string(),
                        "Aktionen".to_string(),
                    ]>
                        {move || {
                            banned_pandas_resource
                                .get()
                                .unwrap()
                                .unwrap()
                                .into_iter()
                                .map(|user| {
                                    view! {
                                        <tr>
                                            <td>{user.display_name.clone()}</td>
                                            <td>{user.email.clone()}</td>
                                            <td>{user.discord_name.clone()}</td>
                                            <td>
                                                <Button
                                                    label="Ban aufheben"
                                                    on:click=move |_| on_unban.call(user.clone())
                                                />
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </Table>
                </Show>
                <h2>{"Gefahrenzone"}</h2>
                <AlertMessage message_type=MessageType::Negative header="Achtung!">
                    <MessageAction slot label="Hain löschen" on_click=on_delete />
                    <MessageContent slot>
                        {"Achtung, hier beginnt die Gefahrenzone, wenn du unten auf Hain löschen klickst wird dein Hain gelöscht. Du wirst nochmal um eine Bestätigung gebeten, danach ist der Hain unwiderbringlich gelöscht."}
                    </MessageContent>
                </AlertMessage>
            </div>
        </Transition>
    }
}
