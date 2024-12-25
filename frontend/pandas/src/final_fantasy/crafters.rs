use crate::api::ff::{get_crafters, CreateCrafterAction, DeleteCrafterAction, EditCrafterAction};
use crate::components::*;
use bamboo_common::core::entities::{Crafter, CrafterJob};
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateCrafterDialog(
    character_id: Signal<i32>,
    available_crafters: Signal<Vec<CrafterJob>>,
    on_close: Callback<(), ()>,
    on_save: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<CreateCrafterAction>::new();
    let selected_job = RwSignal::new(
        available_crafters
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = Memo::new(move |_| {
        available_crafters
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
                    name="crafter_job"
                />
                <Textbox required=false label="Level" name="level" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Sammler hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
fn EditCrafterDialog(
    #[prop(into)] character_id: Signal<i32>,
    #[prop(into)] id: Signal<i32>,
    #[prop(into)] job: Signal<CrafterJob>,
    #[prop(into)] level: Signal<String>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<EditCrafterAction>::new();
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
                    name="crafter_job"
                />
                <Textbox required=false label="Level" name="level" value=level />
            </ModalContent>
            <ModalButton on_click=on_close label="Änderungen verwerfen" slot />
            <ModalButton
                is_submit=true
                label=format!("{} bearbeiten", job.read().to_string())
                slot
            />
        </ActionFormModal>
    }
}

#[component]
pub fn CrafterTab(character_id: Signal<i32>) -> impl IntoView {
    let crafter_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_crafters(id).await },
    );

    let id = RwSignal::new(i32::default());
    let job = RwSignal::new(CrafterJob::default());
    let level = RwSignal::new(String::default());

    let delete_crafter_action = ServerAction::<DeleteCrafterAction>::new();

    let available_crafter = RwSignal::new(vec![]);
    let add_enabled = Memo::new(move |_| !available_crafter.read().is_empty());
    let add_open = RwSignal::new(false);
    let add_saved = Callback::from(move || {
        crafter_resource.refetch();
        add_open.set(false)
    });

    let edit_open = RwSignal::new(false);
    let edit_saved = Callback::from(move || {
        crafter_resource.refetch();
        edit_open.set(false)
    });

    let delete_crafter = move |crafter_id: i32| {
        Suspend::new(async move {
            if let Ok(Some(crafter)) = crafter_resource
                .await
                .map(|res| res.iter().find(|&f| f.id == crafter_id).cloned().clone())
            {
                use_modals().confirm(
                    "Sammler löschen",
                    format!("Soll der Sammler {} wirklich gelöscht werden?", crafter.job),
                    Variant::Negative,
                    format!("{} löschen", crafter.job),
                    format!("{} behalten", crafter.job),
                    Some(Callback::new(move |_| {
                        delete_crafter_action.dispatch(DeleteCrafterAction {
                            crafter_id,
                            character_id: character_id.get(),
                        });
                    })),
                    None,
                );
            }
        });
    };
    let edit_crafter = move |crafter: Crafter| {
        *id.write() = crafter.id;
        *job.write() = crafter.job;
        *level.write() = crafter.level.unwrap_or_default();
        *edit_open.write() = true;
    };

    Effect::new(move |_| {
        if delete_crafter_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            crafter_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-character-tab is--crafter">
                <Show
                    when=move || {
                        crafter_resource
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
                            let crafter_resource = crafter_resource;
                            Suspend::new(async move {
                                crafter_resource
                                    .await
                                    .map(|crafters| {
                                        *available_crafter.write() = {
                                            let used_crafter = crafters
                                                .iter()
                                                .map(|g| g.job)
                                                .collect::<Vec<_>>();
                                            CrafterJob::iter()
                                                .filter(|job| !used_crafter.contains(job))
                                                .collect::<Vec<_>>()
                                        };
                                        crafters
                                            .iter()
                                            .cloned()
                                            .map(|crafter| {
                                                let crafter_to_edit = crafter.clone();

                                                view! {
                                                    <Card
                                                        title=crafter.job.to_string()
                                                        prepend=format!(
                                                            "/pandas/assets/crafter_jobs/{}",
                                                            crafter.job.get_file_name(),
                                                        )
                                                    >
                                                        {if crafter
                                                            .level
                                                            .clone()
                                                            .is_none_or(|level| level.is_empty())
                                                        {
                                                            "Kein Level angegeben".to_string()
                                                        } else {
                                                            format!("Level {}", crafter.level.unwrap())
                                                        }}
                                                        <CardBottom slot>
                                                            <Button
                                                                label="Bearbeiten"
                                                                on:click=move |_| edit_crafter(crafter_to_edit.clone())
                                                            />
                                                            <Button
                                                                label="Löschen"
                                                                on:click=move |_| delete_crafter(crafter.id)
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
                    <CreateCrafterDialog
                        character_id=character_id
                        available_crafters=available_crafter.into()
                        on_close=Callback::from(move || add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
                <Show when=move || edit_open.get()>
                    <EditCrafterDialog
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
