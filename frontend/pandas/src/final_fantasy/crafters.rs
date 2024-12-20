use crate::api::ff::{get_crafters, CreateCrafterAction, DeleteCrafterAction};
use crate::components::*;
use bamboo_common::core::entities::CrafterJob;
use leptos::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateCrafterDialog(
    character_id: MaybeSignal<i32>,
    available_crafters: MaybeSignal<Vec<CrafterJob>>,
    on_close: Callback<(), ()>,
    on_save: Callback<(), ()>,
) -> impl IntoView {
    let action = create_server_action::<CreateCrafterAction>();
    let selected_job = create_rw_signal(
        available_crafters
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = create_memo(move |_| {
        available_crafters
            .get()
            .iter()
            .map(|job| (Some(job.get_job_name()), job.to_string()))
            .collect::<Vec<_>>()
    });

    create_effect(move |_| {
        if action.value().get().is_some() {
            on_save.call(())
        }
    });

    view! {
        <ActionFormModal action=action title="Handwerker hinzufügen">
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect label="Job" items=dropdown_items selected=selected_job name="crafter_job" />
                <Textbox required=false label="Level" name="level" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Handwerker hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn CrafterTab(character_id: MaybeSignal<i32>) -> impl IntoView {
    let crafter_resource = create_local_resource(
        move || character_id.get(),
        |id| async move { get_crafters(id).await },
    );
    let delete_crafter_action = create_server_action::<DeleteCrafterAction>();

    let add_enabled = create_memo(move |_| {
        crafter_resource
            .get()
            .is_some_and(|res| res.is_ok_and(|res| res.len() != CrafterJob::iter().len()))
    });
    let available_crafter = create_memo(move |_| {
        let all_crafter_jobs = CrafterJob::iter().collect::<Vec<_>>();

        if let Some(Ok(crafter)) = crafter_resource.get() {
            let used_crafter = crafter.iter().map(|g| g.job.clone()).collect::<Vec<_>>();
            all_crafter_jobs
                .iter()
                .cloned()
                .filter(|job| !used_crafter.contains(job))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    let add_open = create_rw_signal(false);
    let add_saved = Callback::from(move |_| {
        crafter_resource.refetch();
        add_open.set(false)
    });

    let delete_crafter = {
        let crafter_resource = crafter_resource.clone();
        let delete_crafter_action = delete_crafter_action.clone();

        move |crafter_id: i32| {
            if let Some(Some(Some(crafter))) = crafter_resource.get().map(|res| {
                res.ok()
                    .map(|res| res.iter().cloned().find(|f| f.id == crafter_id).clone())
            }) {
                confirm(
                    "Handwerker löschen",
                    format!(
                        "Soll der Handwerker {} wirklich gelöscht werden?",
                        crafter.job
                    ),
                    Variant::Negative,
                    format!("{} löschen", crafter.job),
                    format!("{} behalten", crafter.job),
                    Some(Callback::new(move |_| {
                        delete_crafter_action.dispatch(DeleteCrafterAction {
                            crafter_id,
                            character_id: character_id.get(),
                        })
                    })),
                    None,
                );
            }
        }
    };

    create_effect(move |_| {
        if delete_crafter_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            crafter_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! {<ProgressRing />}>
            <div class="pandas-character-tab is--crafter">
                <Show when=move || crafter_resource.get().is_some_and(|res| res.is_ok_and(|res| !res.is_empty())) fallback=|| view! {
                    <AlertMessage header="Noch keine Handwerker" message_type=MessageType::Information>
                        <MessageContent slot>
                            "Du hast noch keine Handwerker angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                        </MessageContent>
                    </AlertMessage>
                }>
                    <CardList>
                        {move || {
                            crafter_resource
                                .get()
                                .map(|crafters| {
                                    crafters
                                        .ok()
                                        .map(|crafters| {
                                            crafters
                                                .iter()
                                                .cloned()
                                                .map(|crafter| {
                                                    view! {
                                                        <Card
                                                            title=crafter.job.to_string()
                                                            prepend=format!(
                                                                "/pandas/assets/crafter_jobs/{}",
                                                                crafter.job.get_file_name(),
                                                            )
                                                        >
                                                            {if crafter.level.clone().is_none_or(|level| level.is_empty()) {
                                                                "Kein Level angegeben".to_string()
                                                            } else {
                                                                format!("Level {}", crafter.level.unwrap())
                                                            }}
                                                            <CardBottom slot>
                                                                <Button label="Bearbeiten" />
                                                                <Button label="Löschen" on:click=move |_| delete_crafter(crafter.id) />
                                                            </CardBottom>
                                                        </Card>
                                                    }
                                                })
                                                .collect_view()
                                        })
                                })
                        }}
                    </CardList>
                </Show>
                <Show when=move || add_enabled.get()>
                    <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Handwerker oder Sammler hinzufügen" on:click=move |_| add_open.set(true) />
                </Show>
                <Show when=move || add_open.get()>
                    <CreateCrafterDialog
                        character_id=character_id
                        available_crafters=available_crafter.into()
                        on_close=Callback::from(move |_| add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
