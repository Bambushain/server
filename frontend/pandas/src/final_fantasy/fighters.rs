use crate::api::ff::{get_fighters, CreateFighterAction, DeleteFighterAction};
use crate::components::*;
use bamboo_common::core::entities::FighterJob;
use leptos::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateFighterDialog(
    character_id: MaybeSignal<i32>,
    available_fighters: MaybeSignal<Vec<FighterJob>>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = create_server_action::<CreateFighterAction>();
    let selected_job = create_rw_signal(
        available_fighters
            .get()
            .first()
            .map(|job| job.get_job_name()),
    );
    let dropdown_items = create_memo(move |_| {
        available_fighters
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
        <ActionFormModal action=action title="Kämpfer hinzufügen" >
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect label="Job" items=dropdown_items selected=selected_job name="fighter_job" />
                <Textbox required=false label="Level" name="level" />
                <Textbox required=false label="Gear Score" name="gear_score" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Kämpfer hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn FighterTab(character_id: MaybeSignal<i32>) -> impl IntoView {
    let fighter_resource = create_local_resource(
        move || character_id.get(),
        |id| async move { get_fighters(id).await },
    );
    let delete_fighter_action = create_server_action::<DeleteFighterAction>();

    let add_enabled = create_memo(move |_| {
        fighter_resource
            .get()
            .is_some_and(|res| res.is_ok_and(|res| res.len() != FighterJob::iter().len()))
    });
    let available_fighter = create_memo(move |_| {
        let all_fighter_jobs = FighterJob::iter().collect::<Vec<_>>();

        if let Some(Ok(fighter)) = fighter_resource.get() {
            let used_fighter = fighter.iter().map(|g| g.job.clone()).collect::<Vec<_>>();
            all_fighter_jobs
                .iter()
                .cloned()
                .filter(|job| !used_fighter.contains(job))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    let add_open = create_rw_signal(false);
    let add_saved = Callback::from(move |_| {
        fighter_resource.refetch();
        add_open.set(false)
    });

    let delete_fighter = {
        let fighter_resource = fighter_resource.clone();
        let delete_fighter_action = delete_fighter_action.clone();

        move |fighter_id: i32| {
            if let Some(Some(Some(fighter))) = fighter_resource.get().map(|res| {
                res.ok()
                    .map(|res| res.iter().cloned().find(|f| f.id == fighter_id).clone())
            }) {
                confirm(
                    "Kämpfer löschen",
                    format!("Soll der Kämpfer {} wirklich gelöscht werden?", fighter.job),
                    Variant::Negative,
                    format!("{} löschen", fighter.job),
                    format!("{} behalten", fighter.job),
                    Some(Callback::new(move |_| {
                        delete_fighter_action.dispatch(DeleteFighterAction {
                            fighter_id,
                            character_id: character_id.get(),
                        })
                    })),
                    None,
                );
            }
        }
    };

    create_effect(move |_| {
        if delete_fighter_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            fighter_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! {<ProgressRing />}>
            <div class="pandas-character-tab is--fighter">
                <Show when=move || fighter_resource.get().is_some_and(|res| res.is_ok_and(|res| !res.is_empty())) fallback=|| view! {
                    <AlertMessage header="Noch keine Kämpfer" message_type=MessageType::Information>
                        <MessageContent slot>
                            "Du hast noch keine Kämpfer angelegt, klick unten auf das Plus um deinen Ersten anzulegen"
                        </MessageContent>
                    </AlertMessage>
                }>
                    <CardList>
                        {move || {
                            fighter_resource
                                .get()
                                .map(|fighters| {
                                    fighters
                                        .ok()
                                        .map(|fighters| {
                                            fighters
                                                .iter()
                                                .cloned()
                                                .map(|fighter| {
                                                    view! {
                                                        <Card
                                                            title=fighter.job.to_string()
                                                            prepend=format!(
                                                                "/pandas/assets/fighter_jobs/{}",
                                                                fighter.job.get_file_name(),
                                                            )
                                                        >
                                                            {if fighter.level.clone().is_none_or(|level| level.is_empty()) {
                                                                "Kein Level angegeben".to_string()
                                                            } else {
                                                                format!("Level {}", fighter.level.unwrap())
                                                            }}
                                                            <br />
                                                            {if fighter.gear_score.clone().is_none_or(|level| level.is_empty()) {
                                                                "Kein Gear Score angegeben".to_string()
                                                            } else {
                                                                format!("Gear Score {}", fighter.gear_score.unwrap())
                                                            }}
                                                            <CardBottom slot>
                                                                <Button label="Bearbeiten" />
                                                                <Button label="Löschen" on:click=move |_| delete_fighter(fighter.id) />
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
                    <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Kämpfer erstellen" on:click=move |_| add_open.set(true) />
                </Show>
                <Show when=move || add_open.get()>
                    <CreateFighterDialog
                        character_id=character_id
                        available_fighters=available_fighter.into()
                        on_close=Callback::from(move |_| add_open.set(false))
                        on_save=add_saved
                    />
                </Show>
            </div>
        </Transition>
    }
}
