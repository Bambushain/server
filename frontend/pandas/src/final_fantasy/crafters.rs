use crate::api::ff::{get_crafters, CreateCrafterAction, DeleteCrafterAction};
use crate::components::*;
use bamboo_common::core::entities::CrafterJob;
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
        <ActionFormModal action=action title="Handwerker hinzufügen">
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
            <ModalButton is_submit=true label="Handwerker hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn CrafterTab(character_id: Signal<i32>) -> impl IntoView {
    let crafter_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_crafters(id).await },
    );
    let delete_crafter_action = ServerAction::<DeleteCrafterAction>::new();

    let available_crafter = RwSignal::new(vec![]);
    let add_open = RwSignal::new(false);
    let add_enabled = Memo::new(move |_| !available_crafter.read().is_empty());
    let add_saved = Callback::from(move || {
        crafter_resource.refetch();
        add_open.set(false)
    });

    let delete_crafter = {
        let crafter_resource = crafter_resource.clone();
        let delete_crafter_action = delete_crafter_action.clone();

        move |crafter_id: i32| {
            Suspend::new(async move {
                if let Ok(Some(crafter)) = crafter_resource
                    .await
                    .map(|res| res.iter().cloned().find(|f| f.id == crafter_id).clone())
                {
                    use_modals().confirm(
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
                            });
                        })),
                        None,
                    );
                }
            });
        }
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
                                header="Noch keine Handwerker"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    "Du hast noch keine Handwerker angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                                </MessageContent>
                            </AlertMessage>
                        }
                    }
                >
                    <CardList>
                        {move || {
                            let crafter_resource = crafter_resource.clone();
                            Suspend::new(async move {
                                crafter_resource
                                    .await
                                    .map(|crafters| {
                                        *available_crafter.write() = {
                                            let used_crafter = crafters.iter().map(|g| g.job.clone()).collect::<Vec<_>>();
                                            CrafterJob::iter()
                                                .filter(|job| !used_crafter.contains(job))
                                                .collect::<Vec<_>>()
                                        };
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
                                                            <Button label="Bearbeiten" />
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
                        title="Handwerker hinzufügen"
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
            </div>
        </Transition>
    }
}
