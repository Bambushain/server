use crate::api::ff;
use crate::api::ff::DeleteCharacterAction;

use crate::final_fantasy::crafters::CrafterTab;
use crate::final_fantasy::fighters::FighterTab;
use crate::final_fantasy::gatherer::GathererTab;
use crate::final_fantasy::housings::HousingTab;
use bamboo_common::core::entities::Character;
use leptos::*;
use leptos_cosmo::prelude::*;
use std::collections::BTreeMap;

#[component]
fn DetailsTab(
    character: MaybeSignal<Option<Character>>,
    delete_success: Callback<(), ()>,
) -> impl IntoView {
    let custom_fields = {
        let character = character.clone();

        create_memo(move |_| {
            character
                .get()
                .map(|character| {
                    character
                        .custom_fields
                        .into_iter()
                        .map(|field| {
                            (
                                field.position,
                                (
                                    field.label.clone(),
                                    field.values.into_iter().collect::<Vec<_>>().clone(),
                                ),
                            )
                        })
                        .collect::<BTreeMap<_, _>>()
                })
                .unwrap_or_default()
        })
    };
    let world = {
        let character = character.clone();

        move || {
            character
                .get()
                .map(|character| character.world.clone())
                .unwrap_or_default()
        }
    };
    let race = {
        let character = character.clone();

        move || {
            character
                .get()
                .map(|character| character.race.clone())
                .unwrap_or_default()
        }
    };
    let free_company = {
        let character = character.clone();

        move || {
            character
                .get()
                .map(|character| character.free_company.clone())
                .unwrap_or_default()
        }
    };

    let delete_character_action = create_server_action::<DeleteCharacterAction>();

    let delete_character = {
        let delete_character_action = delete_character_action.clone();

        move || {
            if let Some(character) = character.get() {
                let name = character.name;
                confirm(
                    format!("{name} löschen"),
                    format!("Soll der Charakter {name} wirklich gelöscht werden? Damit werden ebenfalls alle Kämpfer, Handwerker, Sammler und Unterkünfte gelöscht."),
                    Variant::Negative,
                    format!("{name} löschen"),
                    format!("{name} behalten"),
                    Some(Callback::new(move |_| {
                        delete_character_action.dispatch(DeleteCharacterAction {
                            character_id: character.id,
                        })
                    })),
                    None,
                );
            }
        }
    };

    create_effect(move |_| {
        if delete_character_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            delete_success.call(())
        }
    });

    view! {
        <div class="pandas-character-tab">
            <Toolbar>
                <ToolbarGroup>
                    <Button label="Bearbeiten" />
                    <Button label="Löschen" on:click=move |_| delete_character() />
                </ToolbarGroup>
            </Toolbar>
            <div class="pandas-character-tab__content">
                <KeyValueList>
                    <dt>Rasse</dt>
                    <dd>{move || race().to_string()}</dd>
                    <dt>Welt</dt>
                    <dd>{world}</dd>
                    {move || {
                        free_company()
                            .map(|free_company| {
                                view! {
                                    <dt>Freie Gesellschaft</dt>
                                    <dd>{free_company.name.clone()}</dd>
                                }
                            })
                    }}
                    {move || {
                        custom_fields
                            .get()
                            .iter()
                            .map(|(_, (label, values))| {
                                view! {
                                    <dt>{label}</dt>
                                    <dd>{values.join(", ")}</dd>
                                }
                            })
                            .collect_view()
                    }}
                </KeyValueList>
            </div>
        </div>
    }
}

#[component]
pub fn Characters() -> impl IntoView {
    let characters = create_local_resource(|| (), |_| async move { ff::get_characters().await });
    let search_value = create_rw_signal("".to_string());
    let filtered_characters = create_memo(move |_| {
        if let Some(Ok(characters)) = characters.get() {
            characters
                .into_iter()
                .filter(|char| {
                    let lower_search_value = search_value.get().to_lowercase();
                    char.world.to_lowercase().contains(&lower_search_value)
                        || char.name.to_lowercase().contains(&lower_search_value)
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    });

    let has_characters = create_rw_signal(None);

    let selected_character_id = create_rw_signal::<i32>(-1);
    let selected_character = create_memo(move |_| {
        let selected_char = filtered_characters
            .get()
            .iter()
            .cloned()
            .find(|char| char.id == selected_character_id.get());
        if selected_char.is_none() {
            filtered_characters.get().first().cloned()
        } else {
            selected_char
        }
    });
    let details_tab_label = create_memo(move |_| {
        selected_character
            .get()
            .map(|character| format!("Über {}", character.name.clone()))
            .unwrap_or_default()
    });
    let character_name = create_memo(move |_| {
        selected_character
            .get()
            .map(|character| character.name.clone())
            .unwrap_or_default()
    });
    let character_id = create_memo(move |_| {
        selected_character
            .get()
            .map(|character| character.id.clone())
            .unwrap_or_default()
    });
    let character_race = create_memo(move |_| {
        selected_character
            .get()
            .map(|character| character.race.to_string())
            .unwrap_or_default()
    });
    let has_selected_character = create_memo(move |_| selected_character.get().is_some());

    let selected_tab = create_rw_signal(0);

    let delete_success = Callback::new(move |_| characters.refetch());

    create_effect(move |_| {
        if let Some(Ok(characters)) = characters.get() {
            has_characters.set(Some(!characters.is_empty()))
        }
    });

    view! {
        <leptos_meta::Title text="Charaktere" />
        <Show when=move || characters.get().is_some() fallback=|| view! { <ProgressRing /> }>
            <Show
                when=move || characters.get().is_some_and(|res| res.is_ok())
                fallback=|| {
                    view! {
                        <AlertMessage header="Fehler beim Laden" message_type=MessageType::Negative>
                            <MessageContent slot>
                                <p>
                                    "Leider konnten deine Charaktere nicht geladen werden, wende dich bitte an den Bambusssupport."
                                </p>
                            </MessageContent>
                        </AlertMessage>
                    }
                }
            >
                <Show
                    when=move || {
                        characters.get().is_some_and(|res| res.is_ok_and(|res| !res.is_empty()))
                    }
                    fallback=|| {
                        view! {
                            <AlertMessage
                                header="Keine Charaktere"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    <div>
                                        "Du hast noch keine Charaktere erstellt, warum erstellst du dir nicht direkt einen um Bambushain voll und ganz zu nutzen."
                                        <div class="cosmo-button__container">
                                            <Button
                                                label="Neuer Charakter"
                                                variant=Variant::Information
                                            />
                                        </div>
                                    </div>
                                </MessageContent>
                            </AlertMessage>
                        }
                    }
                >
                    <div class="pandas-characters-list">
                        <div class="pandas-characters-list__items">
                            <input
                                type="search"
                                class="cosmo-input pandas-characters-search-bar"
                                placeholder="Nach Name oder Welt filtern…"
                                prop:value=search_value
                                on:input=move |ev| {
                                    search_value.set(event_target_value(&ev).clone())
                                }
                            />
                            <div class="pandas-characters-list__items-inner">
                                <Show
                                    when=move || !filtered_characters.get().is_empty()
                                    fallback=|| {
                                        view! {
                                            <AlertMessage
                                                header="Keine Charaktere gefunden"
                                                message_type=MessageType::Information
                                            >
                                                <MessageContent slot>
                                                    <div>
                                                        Für deine Suche wurden keine passenden Charaktere gefunden.
                                                    </div>
                                                </MessageContent>
                                            </AlertMessage>
                                        }
                                    }
                                >
                                    {filtered_characters
                                        .get()
                                        .iter()
                                        .cloned()
                                        .map(move |character| {
                                            let name = character.name.clone();
                                            let race = character.race.to_string();
                                            let world = character.world.clone();
                                            view! {
                                                <div
                                                    class="pandas-characters-list__item"
                                                    class=(
                                                        "is--active",
                                                        move || {
                                                            selected_character
                                                                .get()
                                                                .is_some_and(|char| char.id == character.id)
                                                        },
                                                    )
                                                    on:click={
                                                        let character = character.clone();
                                                        move |_| selected_character_id.set(character.id)
                                                    }
                                                >
                                                    <span class="pandas-characters-title">{name}</span>
                                                    <span class="pandas-characters-subtitle">
                                                        {format!("{race} auf {world}")}
                                                    </span>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </Show>
                            </div>
                            <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Charakter erstellen" />
                        </div>
                        <div class="pandas-characters-list__separator"></div>
                        <div class="pandas-characters-list__details">
                            <Show when=move || has_selected_character.get()>
                                <Title title=character_name subtitle=character_race />
                                <TabControl selected_index=selected_tab>
                                    <TabItem slot label=details_tab_label>
                                        <DetailsTab character=selected_character.into() delete_success=delete_success />
                                    </TabItem>
                                    <TabItem slot label="Kämpfer">
                                        <FighterTab character_id=character_id.into() />
                                    </TabItem>
                                    <TabItem slot label="Handwerker">
                                        <CrafterTab character_id=character_id.into() />
                                    </TabItem>
                                    <TabItem slot label="Sammler">
                                        <GathererTab character_id=character_id.into() />
                                    </TabItem>
                                    <TabItem slot label="Unterkünfte">
                                        <HousingTab character_id=character_id.into() />
                                    </TabItem>
                                </TabControl>
                            </Show>
                        </div>
                    </div>
                </Show>
            </Show>
        </Show>
    }
}
