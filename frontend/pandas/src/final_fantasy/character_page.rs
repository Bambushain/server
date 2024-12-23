use crate::api::ff;
use crate::api::ff::DeleteCharacterAction;

use crate::final_fantasy::crafters::CrafterTab;
use crate::final_fantasy::fighters::FighterTab;
use crate::final_fantasy::gatherer::GathererTab;
use crate::final_fantasy::housings::HousingTab;
use bamboo_common::core::entities::Character;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use std::collections::BTreeMap;

#[component]
fn DetailsTab(
    character: Signal<Option<Character>>,
    delete_success: Callback<(), ()>,
) -> impl IntoView {
    let custom_fields = Memo::new(move |_| {
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
    });
    let world = Memo::new(move |_| {
        character
            .get()
            .map(|character| character.world.clone())
            .unwrap_or_default()
    });
    let race = Memo::new(move |_| {
        character
            .get()
            .map(|character| character.race.clone().to_string())
            .unwrap_or_default()
    });
    let free_company = Memo::new(move |_| {
        character
            .get()
            .map(|character| character.free_company.clone())
            .unwrap_or_default()
    });

    let delete_character_action = ServerAction::<DeleteCharacterAction>::new();

    let delete_character = {
        let delete_character_action = delete_character_action.clone();

        move || {
            if let Some(character) = character.get() {
                let name = character.name;
                use_modals().confirm(
                    format!("{name} löschen"),
                    format!("Soll der Charakter {name} wirklich gelöscht werden? Damit werden ebenfalls alle Kämpfer, Handwerker, Sammler und Unterkünfte gelöscht."),
                    Variant::Negative,
                    format!("{name} löschen"),
                    format!("{name} behalten"),
                    Some(Callback::new(move |_| {
                        delete_character_action.dispatch(DeleteCharacterAction {
                            character_id: character.id,
                        });
                    })),
                    None,
                );
            }
        }
    };

    Effect::new(move |_| {
        if delete_character_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            delete_success.run(())
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
                    <dd>{race}</dd>
                    <dt>Welt</dt>
                    <dd>{world}</dd>
                    {move || {
                        free_company
                            .get()
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
                            .into_values()
                            .map(|(label, values)| {
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
    let characters_resource = Resource::new(|| (), |_| async move { ff::get_characters().await });
    let search_value = RwSignal::new("".to_string());
    let characters = RwSignal::<Vec<Character>>::new(vec![]);
    let filtered_characters = Memo::new(move |_| {
        characters
            .get()
            .into_iter()
            .filter(|char| {
                let lower_search_value = search_value.get().to_lowercase();
                char.world.to_lowercase().contains(&lower_search_value)
                    || char.name.to_lowercase().contains(&lower_search_value)
            })
            .collect::<Vec<_>>()
    });

    let selected_character_id = RwSignal::new(-1);
    let selected_character = Memo::new(move |_| {
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
    let details_tab_label = Memo::new(move |_| {
        selected_character
            .get()
            .map(|character| format!("Über {}", character.name.clone()))
            .unwrap_or_default()
    });
    let character_name = Memo::new(move |_| {
        selected_character
            .get()
            .map(|character| character.name.clone())
            .unwrap_or_default()
    });
    let character_id = Memo::new(move |_| {
        selected_character
            .get()
            .map(|character| character.id.clone())
            .unwrap_or_default()
    });
    let character_race = Memo::new(move |_| {
        selected_character
            .get()
            .map(|character| character.race.to_string())
            .unwrap_or_default()
    });
    let has_selected_character = Memo::new(move |_| selected_character.read().is_some());

    let selected_tab = RwSignal::new(0);

    let delete_success = Callback::new(move |_| characters_resource.refetch());

    Effect::new(move |_| characters_resource.refetch());

    view! {
        <leptos_meta::Title text="Charaktere" />
        <Suspense fallback=|| {
            view! { <ProgressRing /> }
        }>
            {move || Suspend::new(async move {
                if let Ok(chars) = characters_resource.await {
                    characters.set(chars.clone());
                    Either::Left(
                        view! {
                            <Show
                                when=move || !chars.is_empty()
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
                                            bind:value=search_value
                                        />
                                        <div class="pandas-characters-list__items-inner">
                                            <Show
                                                when=move || !filtered_characters.read().is_empty()
                                                fallback=|| {
                                                    view! {
                                                        <AlertMessage
                                                            header="Keine Charaktere gefunden"
                                                            message_type=MessageType::Information
                                                        >
                                                            <MessageContent slot>
                                                                Für deine Suche wurden keine passenden Charaktere gefunden.
                                                            </MessageContent>
                                                        </AlertMessage>
                                                    }
                                                }
                                            >
                                                {move || {
                                                    filtered_characters
                                                        .get()
                                                        .into_iter()
                                                        .map(move |character| {
                                                            let name = character.name.clone();
                                                            let race = character.race.to_string();
                                                            let world = character.world.clone();
                                                            view! {
                                                                <div
                                                            class="pandas-characters-list__item"
                                                            class:is--active=move || selected_character
                                                                        .get()
                                                                        .is_some_and(|char| char.id == character.id)
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
                                                        .collect_view()
                                                }}
                                            </Show>
                                        </div>
                                        <CircleButton
                                            size=CircleButtonSize::Large
                                            variant=Variant::Primary
                                            icon=icons::LuPlus
                                            title="Charakter erstellen"
                                        />
                                    </div>
                                    <div class="pandas-characters-list__separator"></div>
                                    <div class="pandas-characters-list__details">
                                        <Show when=move || has_selected_character.get()>
                                            <Title title=character_name subtitle=character_race />
                                            <TabControl selected_index=selected_tab>
                                                <TabItem slot label=details_tab_label>
                                                    <DetailsTab
                                                        character=selected_character.into()
                                                        delete_success=delete_success
                                                    />
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
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <AlertMessage
                                header="Fehler beim Laden"
                                message_type=MessageType::Negative
                            >
                                <MessageContent slot>
                                    <p>
                                        "Leider konnten deine Charaktere nicht geladen werden, wende dich bitte an den Bambusssupport."
                                    </p>
                                </MessageContent>
                            </AlertMessage>
                        },
                    )
                }
            })}
        </Suspense>
    }
}
