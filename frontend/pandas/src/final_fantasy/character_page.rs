use crate::api::ff::{
    create_character, get_custom_fields, get_free_companies, update_character,
    DeleteCharacterAction,
};
use crate::api::{ff, BambooCodeError};
use crate::final_fantasy::crafters::CrafterTab;
use crate::final_fantasy::fighters::FighterTab;
use crate::final_fantasy::gatherer::GathererTab;
use crate::final_fantasy::housings::HousingTab;
use bamboo_common::core::entities::{Character, CharacterRace};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_cosmo::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use rand::prelude::IteratorRandom;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;
use strum::IntoEnumIterator;

macro_rules! select_option {
    ($option: expr) => {
        (Some($option.to_string()), $option.to_string())
    };
}

#[component]
fn DataCenterAndWorldDropdowns(
    datacenter: RwSignal<Option<String>>,
    world: RwSignal<Option<String>>,
) -> impl IntoView {
    let datacenters = vec![
        select_option!("Aether"),
        select_option!("Chaos"),
        select_option!("Crystal"),
        select_option!("Dynamis"),
        select_option!("Elemental"),
        select_option!("Gaia"),
        select_option!("Light"),
        select_option!("Mana"),
        select_option!("Materia"),
        select_option!("Meteor"),
        select_option!("Primal"),
    ];
    let worlds = Memo::new(move |_| match datacenter.get().as_deref() {
        Some("Chaos") => vec![
            select_option!("Cerberus"),
            select_option!("Louisoix"),
            select_option!("Moogle"),
            select_option!("Omega"),
            select_option!("Phantom"),
            select_option!("Ragnarok"),
            select_option!("Sagittarius"),
            select_option!("Spriggan"),
        ],
        Some("Light") => vec![
            select_option!("Alpha"),
            select_option!("Lich"),
            select_option!("Odin"),
            select_option!("Phoenix"),
            select_option!("Raiden"),
            select_option!("Shiva"),
            select_option!("Twintania"),
        ],
        Some("Materia") => vec![
            select_option!("Bismarck"),
            select_option!("Ravana"),
            select_option!("Sephiroth"),
            select_option!("Sophia"),
            select_option!("Zurvan"),
        ],
        Some("Aether") => vec![
            select_option!("Adamantoise"),
            select_option!("Cactuar"),
            select_option!("Faerie"),
            select_option!("Gilgamesh"),
            select_option!("Jenova"),
            select_option!("Midgardsormr"),
            select_option!("Sargatanas"),
            select_option!("Siren"),
        ],
        Some("Crystal") => vec![
            select_option!("Balmung"),
            select_option!("Brynhildr"),
            select_option!("Coerul"),
            select_option!("Diabolos"),
            select_option!("Goblin"),
            select_option!("Malboro"),
            select_option!("Mateur"),
            select_option!("Zalera"),
        ],
        Some("Dynamis") => vec![
            select_option!("Cuchulainn"),
            select_option!("Golem"),
            select_option!("Halicarnassus"),
            select_option!("Kraken"),
            select_option!("Maduin"),
            select_option!("Marilith"),
            select_option!("Rafflesia"),
            select_option!("Seraph"),
        ],
        Some("Primal") => vec![
            select_option!("Behemoth"),
            select_option!("Excalibur"),
            select_option!("Exodus"),
            select_option!("Famfrit"),
            select_option!("Hyperion"),
            select_option!("Lamia"),
            select_option!("Leviathan"),
            select_option!("Ultros"),
        ],
        Some("Elemental") => vec![
            select_option!("Aegis"),
            select_option!("Atomos"),
            select_option!("Carbuncle"),
            select_option!("Garuda"),
            select_option!("Gungnir"),
            select_option!("Kujata"),
            select_option!("Tonberry"),
            select_option!("Typhon"),
        ],
        Some("Gaia") => vec![
            select_option!("Alexander"),
            select_option!("Bahamut"),
            select_option!("Durandal"),
            select_option!("Fenrir"),
            select_option!("Ifrit"),
            select_option!("Ridill"),
            select_option!("Tiarmat"),
            select_option!("Ultima"),
        ],
        Some("Mana") => vec![
            select_option!("Anima"),
            select_option!("Asura"),
            select_option!("Chocobo"),
            select_option!("Hades"),
            select_option!("Ixion"),
            select_option!("Masamune"),
            select_option!("Pandaemonium"),
            select_option!("Titan"),
        ],
        Some("Meteor") => vec![
            select_option!("Belias"),
            select_option!("Mandragora"),
            select_option!("Ramuh"),
            select_option!("Shinryu"),
            select_option!("Unicorn"),
            select_option!("Valefor"),
            select_option!("Yojimbo"),
            select_option!("Zeromus"),
        ],
        _ => vec![],
    });

    Effect::new(move |_| {
        *world.write() = match datacenter.get().as_deref() {
            Some("Chaos") => Some("Cerberus".to_string()),
            Some("Light") => Some("Alpha".to_string()),
            Some("Materia") => Some("Bismarck".to_string()),
            Some("Aether") => Some("Adamantoise".to_string()),
            Some("Crystal") => Some("Balmung".to_string()),
            Some("Dynamis") => Some("Cuchulainn".to_string()),
            Some("Primal") => Some("Behemoth".to_string()),
            Some("Elemental") => Some("Aegis".to_string()),
            Some("Gaia") => Some("Alexander".to_string()),
            Some("Mana") => Some("Anima".to_string()),
            Some("Meteor") => Some("Belias".to_string()),
            _ => Some("".to_string()),
        }
    });

    view! {
        <SingleSelect selected=datacenter label="Datenzentrum" name="datacenter" items=datacenters />
        <SingleSelect selected=world label="Stammwelt" name="world" items=worlds />
    }
}

#[component]
fn EditCharacterDialog(
    character: Character,
    on_save: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let has_error = RwSignal::new(false);
    let error_message = RwSignal::new("".to_string());
    let error_message_header = RwSignal::new("".to_string());

    let name = RwSignal::new(character.name);
    let world = RwSignal::new(Some(character.world));
    let datacenter = RwSignal::new(character.datacenter);
    let race = RwSignal::new(Some(character.race.get_race_name()));
    let free_company = RwSignal::new(character.free_company.map(|fc| fc.name));
    let custom_fields = RwSignal::new(
        character
            .custom_fields
            .into_iter()
            .map(|f| {
                (
                    f.label.clone(),
                    RwSignal::new(f.values.into_iter().collect::<Vec<_>>()),
                )
            })
            .collect::<BTreeMap<_, _>>(),
    );

    let custom_fields_resource =
        Resource::new(|| (), move |_| async move { get_custom_fields().await });
    let free_companies_resource =
        Resource::new(|| (), move |_| async move { get_free_companies().await });

    let races = CharacterRace::iter()
        .map(|race| (Some(race.get_race_name()), race.to_string()))
        .collect::<Vec<_>>();

    let update_character = move |ev: SubmitEvent| {
        ev.prevent_default();
        let custom_fields = custom_fields
            .get_untracked()
            .into_iter()
            .map(|(field, values)| {
                (
                    field,
                    values.get_untracked().into_iter().collect::<BTreeSet<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let id = character.id;
        spawn_local(async move {
            *has_error.write() = match update_character(
                id,
                race.get_untracked().unwrap(),
                name.get_untracked(),
                world.get_untracked().unwrap(),
                datacenter.get_untracked().unwrap(),
                free_company.get_untracked(),
                Some(custom_fields),
            )
            .await
            {
                Err(BambooCodeError::ExistsAlready) => {
                    *error_message_header.write() = "Charakter existiert bereits".to_string();
                    *error_message.write() = format!(
                        "Auf der Welt {} existiert bereits ein Charakter mit dem Namen {}",
                        world.get().unwrap(),
                        name.read()
                    );
                    false
                }
                Err(_) => {
                    *error_message_header.write() = "Fehler beim Erstellen".to_string();
                    *error_message.write() = "Ein unbekannter Fehler ist aufgetreten, bitte wende dich an den Bambussupport".to_string();
                    false
                }
                Ok(_) => {
                    on_save.run(());
                    true
                }
            }
        });
    };

    view! {
        <FormModal
            classes="is--character"
            title="Charakter bearbeiten"
            has_error
            error_message
            error_message_header
            on:submit=update_character
        >
            <ModalContent slot>
                <Textbox maxlength=20 label="Name" value=name />
                <SingleSelect label="Rasse" items=races.clone() selected=race />
                <DataCenterAndWorldDropdowns datacenter=datacenter world=world />
                <Transition>
                    {move || {
                        Suspend::new(async move {
                            free_companies_resource
                                .await
                                .map(|fc| {
                                    let mut items = fc
                                        .iter()
                                        .map(|fc| (Some(fc.name.clone()), fc.name.clone()))
                                        .collect::<Vec<_>>();
                                    let mut with_none = vec![
                                        (None, "Keine freie Gesellschaft".to_string()),
                                    ];
                                    with_none.append(&mut items);

                                    view! {
                                        <SingleSelect
                                            label="Freie Gesellschaft"
                                            items=with_none.clone()
                                            selected=free_company
                                        />
                                    }
                                })
                        })
                    }}
                </Transition>
                <Transition>
                    {move || {
                        Suspend::new(async move {
                            custom_fields_resource
                                .await
                                .map(|fields| {
                                    fields
                                        .iter()
                                        .map(|field| {
                                            let items = field
                                                .options
                                                .iter()
                                                .cloned()
                                                .map(|option| (option.label.clone(), option.label.clone()))
                                                .collect::<Vec<_>>();
                                            custom_fields
                                                .try_update(|fields| {
                                                    let new_item = RwSignal::new(vec![]);
                                                    let entry = fields.entry(field.label.clone());
                                                    entry.or_insert(new_item).to_owned()
                                                })
                                                .map(|selected| {
                                                    view! {
                                                        <MultiSelect
                                                            label=field.label.clone()
                                                            items=items
                                                            selected=selected
                                                        />
                                                    }
                                                })
                                        })
                                        .collect_view()
                                })
                        })
                    }}
                </Transition>
            </ModalContent>
            <ModalButton on_click=on_close label="Änderungen verwerfen" slot />
            <ModalButton is_submit=true label="Charakter speichern" slot />
        </FormModal>
    }
}

#[component]
fn CreateCharacterDialog(on_save: Callback<()>, on_close: Callback<()>) -> impl IntoView {
    let has_error = RwSignal::new(false);
    let error_message = RwSignal::new("".to_string());
    let error_message_header = RwSignal::new("".to_string());

    let name = RwSignal::new("".to_string());
    let world = RwSignal::new(Some("Adamantoise".to_string()));
    let datacenter = RwSignal::new(Some("Aether".to_string()));
    let race = RwSignal::new(Some(
        CharacterRace::iter()
            .choose(&mut rand::rng())
            .unwrap_or_default()
            .get_race_name(),
    ));
    let free_company = RwSignal::new(None);
    let custom_fields = RwSignal::new(BTreeMap::<String, RwSignal<Vec<String>>>::new());

    let custom_fields_resource =
        Resource::new(|| (), move |_| async move { get_custom_fields().await });
    let free_companies_resource =
        Resource::new(|| (), move |_| async move { get_free_companies().await });

    let races = CharacterRace::iter()
        .map(|race| (Some(race.get_race_name()), race.to_string()))
        .collect::<Vec<_>>();

    let create_character = move |ev: SubmitEvent| {
        ev.prevent_default();
        let custom_fields = custom_fields
            .get_untracked()
            .into_iter()
            .map(|(field, values)| {
                (
                    field,
                    values.get_untracked().into_iter().collect::<BTreeSet<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        spawn_local(async move {
            *has_error.write() = match create_character(
                race.get_untracked().unwrap(),
                name.get_untracked(),
                world.get_untracked().unwrap(),
                datacenter.get_untracked().unwrap(),
                free_company.get_untracked(),
                Some(custom_fields),
            )
            .await
            {
                Err(BambooCodeError::ExistsAlready) => {
                    *error_message_header.write() = "Charakter existiert bereits".to_string();
                    *error_message.write() = format!(
                        "Auf der Welt {} existiert bereits ein Charakter mit dem Namen {}",
                        world.get().unwrap(),
                        name.read()
                    );
                    false
                }
                Err(_) => {
                    *error_message_header.write() = "Fehler beim Erstellen".to_string();
                    *error_message.write() = "Ein unbekannter Fehler ist aufgetreten, bitte wende dich an den Bambussupport".to_string();
                    false
                }
                Ok(_) => {
                    on_save.run(());
                    true
                }
            }
        });
    };

    view! {
        <FormModal
            classes="is--character"
            title="Charakter hinzufügen"
            has_error
            error_message
            error_message_header
            on:submit=create_character
        >
            <ModalContent slot>
                <Textbox maxlength=20 label="Name" value=name />
                <SingleSelect label="Rasse" items=races.clone() selected=race />
                <DataCenterAndWorldDropdowns datacenter=datacenter world=world />
                <Transition>
                    {move || {
                        Suspend::new(async move {
                            free_companies_resource
                                .await
                                .map(|fc| {
                                    let mut items = fc
                                        .iter()
                                        .map(|fc| (Some(fc.name.clone()), fc.name.clone()))
                                        .collect::<Vec<_>>();
                                    let mut with_none = vec![
                                        (None, "Keine freie Gesellschaft".to_string()),
                                    ];
                                    with_none.append(&mut items);

                                    view! {
                                        <SingleSelect
                                            label="Freie Gesellschaft"
                                            items=with_none.clone()
                                            selected=free_company
                                        />
                                    }
                                })
                        })
                    }}
                </Transition>
                <Transition>
                    {move || {
                        Suspend::new(async move {
                            custom_fields_resource
                                .await
                                .map(|fields| {
                                    fields
                                        .iter()
                                        .map(|field| {
                                            let items = field
                                                .options
                                                .iter()
                                                .cloned()
                                                .map(|option| (option.label.clone(), option.label.clone()))
                                                .collect::<Vec<_>>();
                                            custom_fields
                                                .try_update(|fields| {
                                                    let new_item = RwSignal::new(vec![]);
                                                    fields.insert(field.label.clone(), new_item);
                                                    new_item
                                                })
                                                .map(|selected| {
                                                    view! {
                                                        <MultiSelect
                                                            label=field.label.clone()
                                                            items=items
                                                            selected=selected
                                                        />
                                                    }
                                                })
                                        })
                                        .collect_view()
                                })
                        })
                    }}
                </Transition>
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Charakter hinzufügen" slot />
        </FormModal>
    }
}

#[component]
fn DetailsTab(
    character: Signal<Option<Character>>,
    delete_success: Callback<(), ()>,
    update_success: Callback<(), ()>,
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
            .map(|character| character.world)
            .unwrap_or_default()
    });
    let datacenter = Memo::new(move |_| {
        character
            .get()
            .map(|character| {
                character
                    .datacenter
                    .unwrap_or("Kein Datenzentrum angegeben".to_string())
            })
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

    let edit_open = RwSignal::new(false);

    let delete_character_action = ServerAction::<DeleteCharacterAction>::new();

    let delete_character = {
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

    let update_success = Callback::from(move || {
        edit_open.set(false);
        update_success.run(());
    });
    let update_close = Callback::from(move || edit_open.set(false));

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
                    <Button label="Bearbeiten" on:click=move |_| edit_open.set(true) />
                    <Button label="Löschen" on:click=move |_| delete_character() />
                </ToolbarGroup>
            </Toolbar>
            <div class="pandas-character-tab__content">
                <KeyValueList>
                    <dt>Rasse</dt>
                    <dd>{race}</dd>
                    <dt>Welt</dt>
                    <dd>{world}</dd>
                    <dt>Datenzentrum</dt>
                    <dd>{datacenter}</dd>
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
                    <For
                        each=move || custom_fields.get().into_values()
                        key=move |(label, values)| format!("{label}{values:#?}")
                        let((label, values))
                    >
                        <dt>{label}</dt>
                        <dd>{values.join(", ")}</dd>
                    </For>
                </KeyValueList>
                <Show when=move || edit_open.get()>
                    <EditCharacterDialog
                        character=character.get().unwrap()
                        on_save=update_success
                        on_close=update_close
                    />
                </Show>
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

    let loading = RwSignal::new(false);

    let add_open = RwSignal::new(false);
    let query = use_query_map();

    let selected_character_id = Memo::new(move |_| {
        let first = characters.get().first().map(|first| first.id).unwrap_or(-1);
        query
            .get()
            .get("id")
            .map(|id| i32::from_str(id.as_str()).unwrap_or(first))
            .unwrap_or(first)
    });
    let selected_character = Memo::new(move |_| {
        let selected_char = filtered_characters
            .get()
            .iter()
            .find(|&char| char.id == selected_character_id.get())
            .cloned();
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
            .map(|character| character.id)
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
    let update_success = Callback::new(move |_| {
        loading.set(true);
        characters_resource.refetch()
    });

    let add_saved = Callback::new(move |_| {
        characters_resource.refetch();
        add_open.set(false);
    });
    let open_add_character = Callback::new(move |_| add_open.set(true));

    Effect::new(move |_| characters_resource.refetch());

    view! {
        <leptos_meta::Title text="Charaktere" />
        <div class="pandas-character__page">
            <Transition fallback=|| {
                view! { <ProgressRing /> }
            }>
                {move || Suspend::new(async move {
                    if let Ok(chars) = characters_resource.await {
                        loading.set(false);
                        characters.set(chars.clone());
                        view! {
                            <Show
                                when=move || !chars.is_empty()
                                fallback=move || {
                                    view! {
                                        <AlertMessage
                                            header="Keine Charaktere"
                                            message_type=MessageType::Information
                                        >
                                            <MessageContent slot>
                                                "Du hast noch keine Charaktere erstellt, warum erstellst du dir nicht direkt einen um Bambushain voll und ganz zu nutzen."
                                            </MessageContent>
                                            <MessageAction slot label="Neuer Charakter" on_click=open_add_character />
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
                                                <For
                                                    each=move || filtered_characters.get()
                                                    key=move |character| {
                                                        format!(
                                                            "id={}&race={}&name={}&world={}",
                                                            character.id,
                                                            character.race,
                                                            character.name,
                                                            character.world,
                                                        )
                                                    }
                                                    let(character)
                                                >
                                                    {
                                                        let name = character.name.clone();
                                                        let race = character.race.to_string();
                                                        let world = character.world.clone();
                                                        view! {
                                                            <A
                                                                attr:class="pandas-characters-list__item"
                                                                class:is--active=move || selected_character
                                                                            .get()
                                                                            .is_some_and(|char| char.id == character.id)
                                                                href={
                                                                    let character = character.clone();
                                                                    move || format!("/pandas/final-fantasy?id={}", character.id)
                                                                }
                                                            >
                                                                <span class="pandas-characters-title">{name}</span>
                                                                <span class="pandas-characters-subtitle">
                                                                    {format!("{race} auf {world}")}
                                                                </span>
                                                            </A>
                                                        }
                                                    }
                                                </For>
                                            </Show>
                                        </div>
                                        <CircleButton
                                            size=CircleButtonSize::Large
                                            variant=Variant::Primary
                                            icon=icons::LuPlus
                                            title="Charakter erstellen"
                                            on:click=move |_| open_add_character.run(())
                                        />
                                    </div>
                                    <div class="pandas-characters-list__separator"></div>
                                    <div class="pandas-characters-list__details">
                                        <Show when=move || {
                                            has_selected_character.get() && !loading.get()
                                        }>
                                            <Title title=character_name subtitle=character_race />
                                            <TabControl selected_index=selected_tab>
                                                <TabItem slot label=details_tab_label>
                                                    <DetailsTab
                                                        character=selected_character.into()
                                                        delete_success=delete_success
                                                        update_success=update_success
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
                            <Show when=move || add_open.get()>
                                <CreateCharacterDialog
                                    on_save=add_saved
                                    on_close=Callback::from(move || add_open.set(false))
                                />
                            </Show>
                        }.into_any()
                    } else {
                        view! {
                            <AlertMessage
                                header="Fehler beim Laden"
                                message_type=MessageType::Negative
                            >
                                <MessageContent slot>
                                    "Leider konnten deine Charaktere nicht geladen werden, wende dich bitte an den Bambusssupport."
                                </MessageContent>
                            </AlertMessage>
                        }.into_any()
                    }
                })}
            </Transition>
        </div>
    }
}
