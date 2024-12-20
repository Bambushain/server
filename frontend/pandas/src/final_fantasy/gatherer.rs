use crate::api::ff::{get_gatherers, CreateGathererAction, DeleteGathererAction};
use crate::components::*;
use bamboo_common::core::entities::GathererJob;
use leptos::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateGathererDialog(
    character_id: MaybeSignal<i32>,
    available_gatherers: MaybeSignal<Vec<GathererJob>>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = create_server_action::<CreateGathererAction>();
    let selected_job = create_rw_signal(
        available_gatherers
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = create_memo(move |_| {
        available_gatherers
            .get()
            .iter()
            .map(|job| (Some(job.get_job_name()), job.to_string()))
            .collect::<Vec<_>>()
    });

    let value = action.value();

    create_effect(move |_| {
        if value.get().is_some() {
            on_save.call(())
        }
    });

    view! {
        <ActionFormModal action=action title="Sammler hinzufügen" >
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect label="Job" items=dropdown_items selected=selected_job name="gatherer_job" />
                <Textbox required=false label="Level" name="level" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Sammler hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn GathererTab(character_id: MaybeSignal<i32>) -> impl IntoView {
    let gatherer_resource = create_local_resource(
        move || character_id.get(),
        |id| async move { get_gatherers(id).await },
    );
    let delete_gatherer_action = create_server_action::<DeleteGathererAction>();

    let add_enabled = create_memo(move |_| {
        gatherer_resource
            .get()
            .is_some_and(|res| res.is_ok_and(|res| res.len() != GathererJob::iter().len()))
    });
    let available_gatherer = create_memo(move |_| {
        let all_gatherer_jobs = GathererJob::iter().collect::<Vec<_>>();

        if let Some(Ok(gatherer)) = gatherer_resource.get() {
            let used_gatherer = gatherer.iter().map(|g| g.job.clone()).collect::<Vec<_>>();
            all_gatherer_jobs
                .iter()
                .cloned()
                .filter(|job| !used_gatherer.contains(job))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    let add_open = create_rw_signal(false);
    let add_saved = Callback::from(move |_| {
        gatherer_resource.refetch();
        add_open.set(false)
    });

    let delete_gatherer = {
        let gatherer_resource = gatherer_resource.clone();
        let delete_gatherer_action = delete_gatherer_action.clone();

        move |gatherer_id: i32| {
            if let Some(Some(Some(gatherer))) = gatherer_resource.get().map(|res| {
                res.ok()
                    .map(|res| res.iter().cloned().find(|f| f.id == gatherer_id).clone())
            }) {
                confirm(
                    "Sammler löschen",
                    format!(
                        "Soll der Sammler {} wirklich gelöscht werden?",
                        gatherer.job
                    ),
                    Variant::Negative,
                    format!("{} löschen", gatherer.job),
                    format!("{} behalten", gatherer.job),
                    Some(Callback::new(move |_| {
                        delete_gatherer_action.dispatch(DeleteGathererAction {
                            gatherer_id,
                            character_id: character_id.get(),
                        })
                    })),
                    None,
                );
            }
        }
    };

    create_effect(move |_| {
        if delete_gatherer_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            gatherer_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! {<ProgressRing />}>
            <div class="pandas-character-tab is--gatherer">
                <Show when=move || gatherer_resource.get().is_some_and(|res| res.is_ok_and(|res| !res.is_empty())) fallback=|| view! {
                    <AlertMessage header="Noch keine Sammler" message_type=MessageType::Information>
                        <MessageContent slot>
                            "Du hast noch keine Sammler angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                        </MessageContent>
                    </AlertMessage>
                }>
                    <CardList>
                        {move || {
                            gatherer_resource
                                .get()
                                .map(|gatherers| {
                                    gatherers
                                        .ok()
                                        .map(|gatherers| {
                                            gatherers
                                                .iter()
                                                .cloned()
                                                .map(|gatherer| {
                                                    view! {
                                                        <Card
                                                            title=gatherer.job.to_string()
                                                            prepend=format!(
                                                                "/pandas/assets/gatherer_jobs/{}",
                                                                gatherer.job.get_file_name(),
                                                            )
                                                        >
                                                            {if gatherer.level.clone().is_none_or(|level| level.is_empty()) {
                                                                "Kein Level angegeben".to_string()
                                                            } else {
                                                                format!("Level {}", gatherer.level.unwrap())
                                                            }}
                                                            <CardBottom slot>
                                                                <Button label="Bearbeiten" />
                                                                <Button label="Löschen" on:click=move |_| delete_gatherer(gatherer.id) />
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
                    <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Sammler hinzufügen" on:click=move |_| add_open.set(true) />
                </Show>
                <Show when=move || add_open.get()>
                    <CreateGathererDialog
                        character_id=character_id
                        available_gatherers=available_gatherer.into()
                        on_close=Callback::from(move |_| add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
