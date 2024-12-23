use crate::api::ff::{get_gatherers, CreateGathererAction, DeleteGathererAction};
use crate::components::*;
use bamboo_common::core::entities::GathererJob;
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
pub fn GathererTab(character_id: Signal<i32>) -> impl IntoView {
    let gatherer_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_gatherers(id).await },
    );
    let delete_gatherer_action = ServerAction::<DeleteGathererAction>::new();

    let available_gatherer = RwSignal::new(vec![]);
    let add_enabled = Memo::new(move |_| !available_gatherer.read().is_empty());
    let add_open = RwSignal::new(false);
    let add_saved = Callback::from(move || {
        gatherer_resource.refetch();
        add_open.set(false)
    });

    let delete_gatherer = {
        let gatherer_resource = gatherer_resource.clone();
        let delete_gatherer_action = delete_gatherer_action.clone();

        move |gatherer_id: i32| {
            Suspend::new(async move {
                if let Ok(Some(gatherer)) = gatherer_resource
                    .await
                    .map(|res| res.iter().cloned().find(|f| f.id == gatherer_id).clone())
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
        }
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
                        {move || {
                            let gatherer_resource = gatherer_resource.clone();
                            Suspend::new(async move {
                                gatherer_resource
                                    .await
                                    .map(|gatherers| {
                                        *available_gatherer.write() = {
                                            let used_gatherer = gatherers.iter().map(|g| g.job.clone()).collect::<Vec<_>>();
                                            GathererJob::iter()
                                                .filter(|job| !used_gatherer.contains(job))
                                                .collect::<Vec<_>>()
                                        };
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
                                                            <Button label="Bearbeiten" />
                                                            <Button
                                                                label="Löschen"
                                                                on:click=move |_| delete_gatherer(gatherer.id)
                                                            />
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
                    <CircleButton
                        size=CircleButtonSize::Large
                        variant=Variant::Primary
                        icon=icons::LuPlus
                        title="Sammler hinzufügen"
                        on:click=move |_| add_open.set(true)
                    />
                </Show>
                <Show when=move || add_open.get()>
                    <CreateGathererDialog
                        character_id=character_id
                        available_gatherers=available_gatherer.into()
                        on_close=Callback::from(move || add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
