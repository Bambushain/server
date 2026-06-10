use crate::api::ff::{get_fighters, CreateFighterAction, DeleteFighterAction, EditFighterAction};
use crate::components::*;
use bamboo_common::core::entities::{Fighter, FighterJob};
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateFighterDialog(
    #[prop(into)] character_id: Signal<i32>,
    #[prop(into)] available_fighters: Signal<Vec<FighterJob>>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<CreateFighterAction>::new();
    let selected_job = RwSignal::new(
        available_fighters
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = Memo::new(move |_| {
        available_fighters
            .get()
            .iter()
            .map(|job| (Some(job.get_job_name()), job.to_string()))
            .collect::<Vec<_>>()
    });

    let value = action.value();

    Effect::new(move |_| {
        if value.read().is_some() {
            on_save.run(())
        }
    });

    view! {
        <ActionFormModal action=action title="Kämpfer hinzufügen">
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect
                    label="Job"
                    items=dropdown_items
                    selected=selected_job
                    name="fighter_job"
                />
                <Textbox required=false label="Level" name="level" />
                <Textbox required=false label="Gear Score" name="gear_score" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Kämpfer hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
fn EditFighterDialog(
    #[prop(into)] character_id: Signal<i32>,
    #[prop(into)] id: Signal<i32>,
    #[prop(into)] job: Signal<FighterJob>,
    #[prop(into)] gear_score: Signal<String>,
    #[prop(into)] level: Signal<String>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<EditFighterAction>::new();
    let value = action.value();

    let selected_job = RwSignal::new(Some(job.get().get_job_name()));
    let gear_score = RwSignal::new(gear_score.get());
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
                    name="fighter_job"
                />
                <Textbox required=false label="Level" name="level" value=level />
                <Textbox required=false label="Gear Score" name="gear_score" value=gear_score />
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
pub fn FighterTab(character_id: Signal<i32>) -> impl IntoView {
    let fighter_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_fighters(id).await },
    );
    let delete_fighter_action = ServerAction::<DeleteFighterAction>::new();

    let id = RwSignal::new(i32::default());
    let job = RwSignal::new(FighterJob::default());
    let gear_score = RwSignal::new(String::default());
    let level = RwSignal::new(String::default());

    let available_fighter = RwSignal::new(vec![]);
    let add_open = RwSignal::new(false);
    let add_saved = Callback::from(move || {
        fighter_resource.refetch();
        add_open.set(false)
    });

    let edit_open = RwSignal::new(false);
    let edit_saved = Callback::from(move || {
        fighter_resource.refetch();
        edit_open.set(false)
    });

    let delete_fighter = move |fighter_id: i32| {
        Suspend::new(async move {
            if let Ok(Some(fighter)) = fighter_resource
                .await
                .map(|res| res.iter().find(|&f| f.id == fighter_id).cloned().clone())
            {
                use_modals().confirm(
                    "Kämpfer löschen",
                    format!("Soll der Kämpfer {} wirklich gelöscht werden?", fighter.job),
                    Variant::Negative,
                    format!("{} löschen", fighter.job),
                    format!("{} behalten", fighter.job),
                    Some(Callback::new(move |_| {
                        delete_fighter_action.dispatch(DeleteFighterAction {
                            fighter_id,
                            character_id: character_id.get(),
                        });
                    })),
                    None,
                );
            }
        });
    };
    let edit_fighter = move |fighter: Fighter| {
        *id.write() = fighter.id;
        *job.write() = fighter.job;
        *gear_score.write() = fighter.gear_score.unwrap_or_default();
        *level.write() = fighter.level.unwrap_or_default();
        *edit_open.write() = true;
    };

    Effect::new(move |_| {
        if delete_fighter_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            fighter_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-character-tab is--fighter">
                <Show
                    when=move || {
                        fighter_resource
                            .get()
                            .is_some_and(|res| res.is_ok_and(|res| !res.is_empty()))
                    }
                    fallback=|| {
                        view! {
                            <AlertMessage
                                header="Noch keine Kämpfer"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    "Du hast noch keine Kämpfer angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                                </MessageContent>
                            </AlertMessage>
                        }
                    }
                >
                    <CardList>
                        {move || {
                            let fighter_resource = fighter_resource;
                            Suspend::new(async move {
                                fighter_resource
                                    .await
                                    .map(|fighters| {
                                        available_fighter
                                            .set({
                                                let used_fighter = fighters
                                                    .iter()
                                                    .map(|g| g.job)
                                                    .collect::<Vec<_>>();
                                                if fighters.is_empty() {
                                                    FighterJob::iter().collect::<Vec<_>>()
                                                } else {
                                                    FighterJob::iter()
                                                        .filter(|job| !used_fighter.contains(job))
                                                        .collect::<Vec<_>>()
                                                }
                                            });

                                        view! {
                                            <For
                                                each=move || fighters.clone()
                                                key=move |fighter| fighter.clone()
                                                let(fighter)
                                            >
                                                {
                                                    let fighter_to_edit = fighter.clone();

                                                    view! {
                                                        <Card
                                                            title=fighter.job.to_string()
                                                            prepend=format!(
                                                                "/pandas/assets/fighter_jobs/{}",
                                                                fighter.job.get_file_name(),
                                                            )
                                                        >
                                                            {if fighter
                                                                .level
                                                                .clone()
                                                                .is_none_or(|level| level.is_empty())
                                                            {
                                                                "Kein Level angegeben".to_string()
                                                            } else {
                                                                format!("Level {}", fighter.level.unwrap())
                                                            }}
                                                            <br />
                                                            {if fighter
                                                                .gear_score
                                                                .clone()
                                                                .is_none_or(|level| level.is_empty())
                                                            {
                                                                "Kein Gear Score angegeben".to_string()
                                                            } else {
                                                                format!("Gear Score {}", fighter.gear_score.unwrap())
                                                            }}
                                                            <CardBottom slot>
                                                                <Button
                                                                    label="Bearbeiten"
                                                                    on:click=move |_| edit_fighter(fighter_to_edit.clone())
                                                                />
                                                                <Button
                                                                    label="Löschen"
                                                                    on:click=move |_| delete_fighter(fighter.id)
                                                                />
                                                            </CardBottom>
                                                        </Card>
                                                    }
                                                }
                                            </For>
                                        }
                                    })
                            })
                        }}
                    </CardList>
                </Show>
                {
                    let fighter_resource = fighter_resource;
                    move || {
                        Suspend::new(async move {
                            fighter_resource
                                .await
                                .map(|fighters| {
                                    {
                                        let fighters = fighters.clone();
                                        available_fighter
                                            .update(move |old| {
                                                let all_fighters = FighterJob::iter().collect::<Vec<_>>();
                                                let used_fighter = fighters
                                                    .iter()
                                                    .map(|g| g.job)
                                                    .collect::<Vec<_>>();
                                                let new = if fighters.is_empty() {
                                                    all_fighters.clone().to_vec()
                                                } else {
                                                    FighterJob::iter()
                                                        .filter(|job| !used_fighter.contains(job))
                                                        .collect::<Vec<_>>()
                                                };
                                                *old = new;
                                            });
                                    }
                                    (fighters.len() != FighterJob::iter().count())
                                        .then_some(

                                            view! {
                                                <CircleButton
                                                    size=CircleButtonSize::Large
                                                    variant=Variant::Primary
                                                    icon=icons::LuPlus
                                                    title="Kämpfer hinzufügen"
                                                    on:click=move |_| add_open.set(true)
                                                />
                                            },
                                        )
                                })
                        })
                    }
                }
                <Show when=move || add_open.get()>
                    <CreateFighterDialog
                        character_id=character_id
                        available_fighters=available_fighter
                        on_close=Callback::from(move || add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
                <Show when=move || edit_open.get()>
                    <EditFighterDialog
                        character_id=character_id
                        id=id
                        job=job
                        gear_score=gear_score
                        level=level
                        on_close=Callback::from(move || edit_open.set(false))
                        on_save=edit_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
