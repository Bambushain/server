use crate::api::ff::{
    get_gatherers, CreateGathererAction, DeleteGathererAction, EditGathererAction,
};
use crate::components::*;
use bamboo_common::core::entities::{Gatherer, GathererJob};
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateGathererDialog(
    character_id: Signal<i32>,
    available_gatherers: Signal<Vec<GathererJob>>,
    on_close: Callback<(), ()>,
    on_save: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<CreateGathererAction>::new();
    let selected_job = RwSignal::new(
        available_gatherers
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = Memo::new(move |_| {
        available_gatherers
            .get()
            .iter()
            .map(|job| (Some(job.get_job_name()), job.to_string()))
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        if action.value().get().is_some() {
            on_save.run(())
        }
    });

    view! {
        <ActionFormModal action=action title="Sammler hinzufügen">
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect
                    label="Job"
                    items=dropdown_items
                    selected=selected_job
                    name="gatherer_job"
                />
                <Textbox required=false label="Level" name="level" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Sammler hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
fn EditGathererDialog(
    #[prop(into)] character_id: Signal<i32>,
    #[prop(into)] id: Signal<i32>,
    #[prop(into)] job: Signal<GathererJob>,
    #[prop(into)] level: Signal<String>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<EditGathererAction>::new();
    let value = action.value();

    let selected_job = RwSignal::new(Some(job.get().get_job_name()));
    let level = RwSignal::new(level.get());

    Effect::new(move |_| {
        if value.read().is_some() {
            on_save.run(())
        }
    });

    view! {
        <ActionFormModal action=action title=format!("{} bearbeiten", job.read().to_string())>
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <input type="hidden" value=id name="id" />
                <SingleSelect
                    label="Job"
                    items=vec![(Some(job.get().get_job_name()), job.get().to_string())]
                    selected=selected_job
                    name="gatherer_job"
                />
                <Textbox required=false label="Level" name="level" value=level />
            </ModalContent>
            <ModalButton on_click=on_close label="Änderungen verwerfen" slot />
            <ModalButton
                is_submit=true
                label=format!("{} speichern", job.read().to_string())
                slot
            />
        </ActionFormModal>
    }
}

#[component]
pub fn GathererTab(character_id: Signal<i32>) -> impl IntoView {
    let gatherer_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_gatherers(id).await },
    );

    let id = RwSignal::new(i32::default());
    let job = RwSignal::new(GathererJob::default());
    let level = RwSignal::new(String::default());

    let delete_gatherer_action = ServerAction::<DeleteGathererAction>::new();

    let available_gatherer = RwSignal::new(vec![]);
    let add_open = RwSignal::new(false);
    let add_saved = Callback::from(move || {
        gatherer_resource.refetch();
        add_open.set(false)
    });

    let edit_open = RwSignal::new(false);
    let edit_saved = Callback::from(move || {
        gatherer_resource.refetch();
        edit_open.set(false)
    });

    let delete_gatherer = move |gatherer_id: i32| {
        Suspend::new(async move {
            if let Ok(Some(gatherer)) = gatherer_resource
                .await
                .map(|res| res.iter().find(|&f| f.id == gatherer_id).cloned().clone())
            {
                use_modals().confirm(
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
                        });
                    })),
                    None,
                );
            }
        });
    };
    let edit_gatherer = move |gatherer: Gatherer| {
        *id.write() = gatherer.id;
        *job.write() = gatherer.job;
        *level.write() = gatherer.level.unwrap_or_default();
        *edit_open.write() = true;
    };

    Effect::new(move |_| {
        if delete_gatherer_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            gatherer_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-character-tab is--gatherer">
                <Show
                    when=move || {
                        gatherer_resource
                            .get()
                            .is_some_and(|res| res.is_ok_and(|res| !res.is_empty()))
                    }
                    fallback=|| {
                        view! {
                            <AlertMessage
                                header="Noch keine Sammler"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    "Du hast noch keine Sammler angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                                </MessageContent>
                            </AlertMessage>
                        }
                    }
                >
                    <CardList>
                        {
                            let gatherer_resource = gatherer_resource;
                            move || {
                                Suspend::new(async move {
                                    gatherer_resource
                                        .await
                                        .map(|gatherers| {

                                            view! {
                                                <For
                                                    each=move || gatherers.clone()
                                                    key=move |gatherer| gatherer.clone()
                                                    let(gatherer)
                                                >
                                                    {
                                                        let gatherer_to_edit = gatherer.clone();

                                                        view! {
                                                            <Card
                                                                title=gatherer.job.to_string()
                                                                prepend=format!(
                                                                    "/pandas/assets/gatherer_jobs/{}",
                                                                    gatherer.job.get_file_name(),
                                                                )
                                                            >
                                                                {if gatherer
                                                                    .level
                                                                    .clone()
                                                                    .is_none_or(|level| level.is_empty())
                                                                {
                                                                    "Kein Level angegeben".to_string()
                                                                } else {
                                                                    format!("Level {}", gatherer.level.unwrap())
                                                                }}
                                                                <CardBottom slot>
                                                                    <Button
                                                                        label="Bearbeiten"
                                                                        on:click=move |_| edit_gatherer(gatherer_to_edit.clone())
                                                                    />
                                                                    <Button
                                                                        label="Löschen"
                                                                        on:click=move |_| delete_gatherer(gatherer.id)
                                                                    />
                                                                </CardBottom>
                                                            </Card>
                                                        }
                                                    }
                                                </For>
                                            }
                                        })
                                })
                            }
                        }
                    </CardList>
                </Show>
                {
                    let gatherer_resource = gatherer_resource;
                    move || {
                        Suspend::new(async move {
                            gatherer_resource
                                .await
                                .map(|gatherers| {
                                    {
                                        let gatherers = gatherers.clone();
                                        available_gatherer
                                            .update(move |old| {
                                                let all_gatherers = GathererJob::iter().collect::<Vec<_>>();
                                                let used_gatherer = gatherers
                                                    .iter()
                                                    .map(|g| g.job)
                                                    .collect::<Vec<_>>();
                                                let new = if gatherers.is_empty() {
                                                    all_gatherers.clone().to_vec()
                                                } else {
                                                    GathererJob::iter()
                                                        .filter(|job| !used_gatherer.contains(job))
                                                        .collect::<Vec<_>>()
                                                };
                                                *old = new;
                                            });
                                    }
                                    (gatherers.len() != GathererJob::iter().count())
                                        .then_some(

                                            view! {
                                                <CircleButton
                                                    size=CircleButtonSize::Large
                                                    variant=Variant::Primary
                                                    icon=icons::LuPlus
                                                    title="Sammler hinzufügen"
                                                    on:click=move |_| add_open.set(true)
                                                />
                                            },
                                        )
                                })
                        })
                    }
                }
                <Show when=move || add_open.get()>
                    <CreateGathererDialog
                        character_id=character_id
                        available_gatherers=available_gatherer.into()
                        on_close=Callback::from(move || add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
                <Show when=move || edit_open.get()>
                    <EditGathererDialog
                        character_id=character_id
                        id=id
                        job=job
                        level=level
                        on_close=Callback::from(move || edit_open.set(false))
                        on_save=edit_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
