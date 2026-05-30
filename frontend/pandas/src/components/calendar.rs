use crate::api::{create_event, get_events, update_event, DeleteEventAction};
use crate::state::AllGroves;
use bamboo_common::core::entities::{BambooUser, GroveEvent};
use bamboo_common::core::queueing::EventType;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use icondata_lu::{LuPlus, LuTrash};
use leptos::ev::{MouseEvent, SubmitEvent};
use leptos::html::Div;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_cosmo::icons::Icon;
use leptos_cosmo::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use rand::prelude::IndexedRandom;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Not, Sub};
use strum::{EnumIter, IntoEnumIterator};

enum ColorYiqResult {
    Light,
    Dark,
}

#[derive(EnumIter)]
enum Colors {
    ColorE57373,
    ColorF06292,
    ColorBA68C8,
    Color9575CD,
    Color7986CB,
    Color64B5F6,
    Color4FC3F7,
    Color4DD0E1,
    Color4DB6AC,
    Color81C784,
    ColorAED581,
    ColorDCE775,
    ColorFFF176,
    ColorFFD54F,
    ColorFFB74D,
    ColorFF8A65,
}

impl Colors {
    fn get_hex_code(&self) -> &str {
        match self {
            Colors::ColorE57373 => "#E57373",
            Colors::ColorF06292 => "#F06292",
            Colors::ColorBA68C8 => "#BA68C8",
            Colors::Color9575CD => "#9575CD",
            Colors::Color7986CB => "#7986CB",
            Colors::Color64B5F6 => "#64B5F6",
            Colors::Color4FC3F7 => "#4FC3F7",
            Colors::Color4DD0E1 => "#4DD0E1",
            Colors::Color4DB6AC => "#4DB6AC",
            Colors::Color81C784 => "#81C784",
            Colors::ColorAED581 => "#AED581",
            Colors::ColorDCE775 => "#DCE775",
            Colors::ColorFFF176 => "#FFF176",
            Colors::ColorFFD54F => "#FFD54F",
            Colors::ColorFFB74D => "#FFB74D",
            Colors::ColorFF8A65 => "#FF8A65",
        }
    }
}

impl Display for ColorYiqResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ColorYiqResult::Light => "#ffffff",
            ColorYiqResult::Dark => "#333333",
        })
    }
}

fn color_yiq(color: &str) -> ColorYiqResult {
    let color = Color::from_hex(color).unwrap();
    let yiq =
        ((color.red() as u32 * 299) + (color.green() as u32 * 587) + (color.blue() as u32 * 114))
            / 1000;

    if yiq >= 128 {
        ColorYiqResult::Dark
    } else {
        ColorYiqResult::Light
    }
}

#[component]
fn EventEntry(event: GroveEvent) -> impl IntoView {
    let edit_event_open = RwSignal::new(false);

    let color = Color::from_hex(&event.color).unwrap_or_default();

    let me = expect_context::<RwSignal<BambooUser>>();

    let can_edit = {
        let event = event.clone();
        Memo::new(move |_| {
            event
                .user
                .clone()
                .is_some_and(|user| user.id != me.read().id)
        })
    };

    view! {
        <div
            class="pandas-calendar__event"
            style:--event-background-color=color.fade(0.8).hsla()
            style:--event-shadow-color=color.fade(0.9).hsla()
            style:--event-text-color=color_yiq(&event.color).to_string()
        >
            {event.title.clone()}
            <button hidden=can_edit on:click=move |_| edit_event_open.set(true) class="pandas-calendar__event-edit is--button">
                <Icon
                    icon=icondata_lu::LuPencil
                    width="1rem"
                    height="1rem"
                    attr:class="pandas-calendar__event-edit is--icon"
                />
            </button>
            <div class="pandas-calendar__event-description">
                <hgroup class="pandas-calendar__event-header">
                    <h3>{event.title.clone()}</h3>
                    <h5>
                        {if let Some(grove) = event.grove.clone() {
                            { grove.name }
                        } else {
                            "Privates Event".to_string()
                        }}
                    </h5>
                </hgroup>
                {if let (Some(start_time), Some(end_time)) = (event.start_time, event.end_time) {
                    view! {
                        <p>{format!("{} - {}", start_time.format("%H:%M"), end_time.format("%H:%M"))}</p>
                    }.into_any()
                } else {
                   "".into_any()
                }}
                <p>{event.description.clone()}</p>
                <span class="panda-calendar__event-arrow" />
            </div>
        </div>
        <EditEventDialog event=event.clone() is_open=edit_event_open />
    }
}

#[component]
fn Day(
    day: u32,
    month: u32,
    year: i32,
    #[prop(into)] selected_month: Signal<u32>,
    #[prop(into)] events: Signal<Vec<GroveEvent>>,
    #[prop(into)] grove_id: Signal<Option<i32>>,
) -> impl IntoView {
    let add_event_open = RwSignal::new(false);

    let today = Local::now().date_naive();
    let background_color = if selected_month.get() == month {
        "transparent"
    } else {
        "var(--day-background-past-month)"
    };
    let day_number_color = if today.month() == month && today.day() == day && today.year() == year {
        "var(--black)"
    } else {
        "var(--menu-text-color)"
    };

    view! {
        <div
            class="pandas-calendar__day"
            style:--day-number-color=day_number_color
            style:--background-color=background_color
            style:--text=format!("'{}'", day.to_string())
        >
            <Icon
                icon=icons::LuCalendarPlus
                height="1.5rem"
                width="1.5rem"
                attr:class="pandas-calendar__event-add"
                on:click=move |_| add_event_open.set(true)
            />
            <Show when=move || add_event_open.get()>
                <AddEventDialog
                    day=NaiveDate::from_ymd_opt(year, month, day).unwrap()
                    grove_id=grove_id
                    is_open=add_event_open
                />
            </Show>
            <For each=move || events.get() key=move |evt| evt.clone() let(evt)>
                <EventEntry event=evt.clone() />
            </For>
        </div>
    }
}

#[component]
fn BambooColorSelect(
    #[prop(optional, into)] selected: RwSignal<String>,
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] name: Signal<String>,
    #[prop(default = InputWidth::Auto.into(), into)] width: Signal<InputWidth>,
) -> impl IntoView {
    let id = uuid::Uuid::new_v4().to_string();

    let flyout_open = RwSignal::new(false);
    let flyout_up = RwSignal::new(false);

    let select_node = NodeRef::<Div>::new();

    let items = Memo::new(|_| {
        Colors::iter()
            .map(|color| {
                (
                    color.get_hex_code().to_string(),
                    color.get_hex_code().to_string(),
                )
            })
            .collect::<Vec<_>>()
    });

    #[cfg(not(feature = "ssr"))]
    let on_toggle_flyout = move |ev: MouseEvent| {
        let element = event_target::<leptos::web_sys::HtmlElement>(&ev);
        flyout_up.set(
            gloo_utils::window()
                .inner_height()
                .expect("No window? Then this app won't work")
                .as_f64()
                .expect("This should be a number")
                - element.get_bounding_client_rect().bottom()
                < 100.0,
        );

        flyout_open.set(!flyout_open.get());
    };
    #[cfg(feature = "ssr")]
    let on_toggle_flyout = move |_ev: MouseEvent| {};

    let on_select = move |item| {
        selected.set(item);
    };

    let selected_item = {
        move || {
            items
                .read()
                .iter()
                .find(|&(value, _)| selected.read() == *value)
                .cloned()
                .map(|(_, label)| label)
        }
    };

    #[cfg(not(feature = "ssr"))]
    let _ = leptos_use::on_click_outside(select_node, move |_| flyout_open.set(false));

    view! {
        <label for=id class="cosmo-label" on:click=on_toggle_flyout>
            {label}
        </label>
        <div node_ref=select_node class=move || format!("cosmo-select cosmo-input {}", width.read()) on:click=on_toggle_flyout>
            <select style="display:none" name=name prop:value=selected>
                <For each=move || items.get() key=move |(value, ..)| value.clone() let((value, label))>
                    <option prop:value=value>{label}</option>
                </For>
            </select>
            <div
                class="cosmo-select__chip-holder is--color"
                style:--color-value=selected_item
            >
            </div>
            <Show when=move || *flyout_open.read()>
                <div class="cosmo-select__flyout is--color" class:is--up=flyout_up>
                    <For each=move || items.get() key=move |(value, ..)| value.clone() let((value, ..))>
                        <span
                            style:--color-value=value.clone()
                            on:click=move |_| on_select(value.clone())
                            class="cosmo-select__flyout-item is--color"
                        >
                        </span>
                    </For>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn AddEventDialog(
    day: NaiveDate,
    grove_id: Signal<Option<i32>>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let groves = expect_context::<RwSignal<AllGroves>>();

    let start_date = RwSignal::new(day);
    let end_date = RwSignal::new(day);
    let start_time = RwSignal::new(NaiveTime::default());
    let end_time = RwSignal::new(NaiveTime::default());

    let title = RwSignal::new("".to_string());
    let description = RwSignal::new("".to_string());

    let has_time = RwSignal::new(false);

    let selected_grove = RwSignal::new(grove_id.read().map_or(
        groves.read().first().map(|grove| grove.id.to_string()),
        |id| {
            groves
                .read()
                .iter()
                .find(|grove| grove.id == id)
                .map(|grove| grove.name.clone())
        },
    ));

    let notifications = RwSignal::new(vec![]);
    let new_notification = RwSignal::new(
        day.checked_sub_days(Days::new(1))
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .format("%FT%R")
            .to_string(),
    );

    let colors = Colors::iter().collect::<Vec<_>>();

    let color = colors
        .choose(&mut rand::rng())
        .expect("There is at least one color available")
        .get_hex_code()
        .to_string();
    let color = RwSignal::new(color);

    let is_private = RwSignal::new(grove_id.read().is_none());

    let current_grove_id = Memo::new(move |_| {
        groves
            .read()
            .iter()
            .find(|&grove| grove.name == selected_grove.get().unwrap_or("".to_string()))
            .cloned()
    });

    let groves_options = move || {
        groves
            .read()
            .iter()
            .map(|grove| (Some(grove.id.to_string()), grove.name.clone()))
            .collect::<Vec<_>>()
    };

    let has_error = RwSignal::new(false);

    let add_notification = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !new_notification.read().is_empty() {
            notifications.update(|notifications| {
                let notification =
                    NaiveDateTime::parse_from_str(&new_notification.get(), "%FT%R").unwrap();
                notifications.push(notification.and_utc());
            });
            new_notification.set("".to_string());
        }
    };
    let add_event = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let start_time = if has_time.read() == true {
                Some(start_time.get_untracked())
            } else {
                None
            };
            let end_time = if has_time.read() == true {
                Some(end_time.get_untracked())
            } else {
                None
            };

            has_error.set(
                match create_event(
                    title.get_untracked(),
                    Some(description.get_untracked()),
                    color.get_untracked(),
                    start_date.get_untracked(),
                    end_date.get_untracked(),
                    start_time,
                    end_time,
                    is_private.get_untracked(),
                    grove_id.get_untracked(),
                    notifications.get_untracked(),
                )
                .await
                {
                    Err(_) => true,
                    Ok(_) => {
                        is_open.set(false);
                        false
                    }
                },
            );
        });
    };
    let delete_notification = {
        Callback::new(move |time| {
            notifications.update(|values| values.retain(|when| when != &time));
        })
    };

    view! {
        <form id="new_notification" on:submit=add_notification />
        <FormModal
            on:submit=add_event
            title="Event hinzufügen"
            has_error=has_error
            error_message="Das Event konnte leider nicht hinzugefügt werden, bitte wende dich an den Bambussupport."
            error_message_header="Fehler beim Hinzufügen"
        >
            <ModalContent slot>
                <input type="hidden" id="" />
                <Show when=move || current_grove_id.read().is_some()>
                    <input
                        type="hidden"
                        name="grove"
                        value=move || current_grove_id.get().unwrap().id
                    />
                </Show>
                <Textbox width=InputWidth::Medium label="Titel" required=true name="title" value=title />
                <Textarea
                    width=InputWidth::Medium
                    label="Beschreibung"
                    name="description"
                    required=false
                    value=description
                />
                <BambooColorSelect
                    label="Farbe"
                    name="color"
                    selected=color
                />
                <DatePicker
                    width=InputWidth::Medium
                    label="Von"
                    readonly=true
                    name="start_date"
                    value=start_date
                />
                <DatePicker
                    width=InputWidth::Medium
                    label="Bis"
                    required=true
                    min=Some(day)
                    name="end_date"
                    value=end_date
                />
                <Switch label="Mit Zeiten" checked=has_time />
                <Show when=move || *has_time.read()>
                    <TimePicker
                        width=InputWidth::Medium
                        label="Startzeit"
                        required=true
                        name="start_time"
                        value=start_time
                    />
                    <TimePicker
                        width=InputWidth::Medium
                        label="Endzeit"
                        required=true
                        name="end_time"
                        value=end_time
                    />
                </Show>
                <Show when=move || grove_id.read().is_none()>
                    <Switch label="Nur für mich" checked=is_private name="is_private" />
                </Show>
                <Show when=move || grove_id.read().is_none() && is_private.read().not()>
                    <SingleSelect
                        label="Hain"
                        required=true
                        items=groves_options()
                        name="grove"
                        selected=selected_grove
                    />
                </Show>
                <span class="cosmo-input__header">Benachrichtigungen</span>
                <label class="cosmo-label" for="new-option">
                    Zeitpunkt
                </label>
                <div class="cosmo-input is--group">
                    <input
                        class="cosmo-input"
                        id="new-option"
                        type="datetime-local"
                        bind:value=new_notification
                        form="new_notification"
                    />
                    <button
                        type="submit"
                        class="cosmo-button is--primary is--addon"
                        form="new_notification"
                    >
                        <Icon width="1rem" height="1rem" icon=LuPlus />
                    </button>
                </div>
                {move || {
                    notifications
                        .get()
                        .into_iter()
                        .map(|when| {
                            view! {
                                <div class="pandas-custom-fields__option is--new">
                                    <span>{when.format_localized("%A %e %B %Y, %R", Locale::de_DE).to_string()}</span>
                                    <button
                                        type="button"
                                        class="cosmo-button is--negative is--custom-fields is--edit"
                                        on:click=move |_| delete_notification.run(when)
                                    >
                                        <Icon width="1.25rem" height="1.25rem" icon=LuTrash />
                                    </button>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </ModalContent>
            <ModalButton label="Abbrechen" on_click=move || is_open.set(false) slot />
            <ModalButton label="Event speichern" is_submit=true slot />
        </FormModal>
    }
}

#[component]
fn EditEventDialog(event: GroveEvent, is_open: RwSignal<bool>) -> impl IntoView {
    let delete_action = ServerAction::<DeleteEventAction>::new();

    let delete_value = delete_action.value();

    let has_update_error = RwSignal::new(false);
    let has_delete_error = move || delete_value.with(|val| matches!(val, Some(Err(_))));

    Effect::new(move |_| {
        if delete_value.read().is_some() {
            is_open.set(false);
        }
    });

    let grove_id = event.grove.map(|g| g.id);
    let id = event.id;
    let day = event.start_date;

    let title = RwSignal::new(event.title);
    let description = RwSignal::new(event.description);
    let start_date = RwSignal::new(event.start_date);
    let end_date = RwSignal::new(event.end_date);
    let start_time = RwSignal::new(event.start_time.unwrap_or_default());
    let end_time = RwSignal::new(event.end_time.unwrap_or_default());
    let color = RwSignal::new(event.color);

    let has_time = RwSignal::new(event.start_time.is_some());

    let has_error = RwSignal::new(false);

    let notifications = RwSignal::new(
        event
            .reminder
            .into_iter()
            .map(|notification| notification.when)
            .collect::<Vec<DateTime<Utc>>>(),
    );
    let new_notification = RwSignal::new(
        day.checked_sub_days(Days::new(1))
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .format("%FT%R")
            .to_string(),
    );
    let add_notification = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !new_notification.read().is_empty() {
            notifications.update(|notifications| {
                let notification =
                    NaiveDateTime::parse_from_str(&new_notification.get(), "%FT%R").unwrap();
                notifications.push(notification.and_utc());
            });
            new_notification.set("".to_string());
        }
    };
    let save_event = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let start_time = if has_time.read() == true {
                Some(end_time.get_untracked())
            } else {
                None
            };
            let end_time = if has_time.read() == true {
                Some(end_time.get_untracked())
            } else {
                None
            };

            has_error.set(
                match update_event(
                    event.id,
                    title.get_untracked(),
                    Some(description.get_untracked()),
                    color.get_untracked(),
                    start_date.get_untracked(),
                    end_date.get_untracked(),
                    start_time,
                    end_time,
                    notifications.get_untracked(),
                )
                .await
                {
                    Err(_) => true,
                    Ok(_) => {
                        is_open.set(false);
                        false
                    }
                },
            );
        });
    };
    let delete_notification = {
        Callback::new(move |time: DateTime<Utc>| {
            notifications.update(|values| values.retain(|when| when != &time));
        })
    };
    let delete_event = move || {
        use_modals().confirm(
            "Event löschen",
            format!("Soll das Event {} wirklich gelöscht werden?", title.read()),
            Variant::Negative,
            "Event löschen",
            "Nicht löschen",
            Some(Callback::new(move |_| {
                delete_action.dispatch(DeleteEventAction { id });
            })),
            None,
        );
    };

    view! {
        <Show when=move || *is_open.read()>
            <form id="new_notification" on:submit=add_notification />
            <FormModal
                on:submit=save_event
                title="Event hinzufügen"
                has_error=has_update_error
                error_message="Das Event konnte leider nicht gespeichert werden, bitte wende dich an den Bambussupport."
                error_message_header="Fehler beim Speichern"
            >
                <ModalContent slot>
                    <Show when=has_delete_error>
                        <AlertMessage header="Fehler beim Löschen">
                            <MessageContent slot>
                                Das Event konnte leider nicht gelöscht werden, bitte wende dich an den Bambussupport.
                            </MessageContent>
                        </AlertMessage>
                    </Show>
                    <input type="hidden" prop:value=event.id name="id" />
                    <input type="hidden" prop:value=grove_id name="grove_id" />
                    <Textbox width=InputWidth::Medium label="Titel" name="title" value=title />
                    <Textarea
                        width=InputWidth::Medium
                        label="Beschreibung"
                        name="description"
                        value=description
                        required=false
                    />
                    <BambooColorSelect
                        label="Farbe"
                        name="color"
                        selected=color
                    />
                    <DatePicker
                        width=InputWidth::Medium
                        label="Von"
                        readonly=true
                        name="start_date"
                        value=start_date
                    />
                    <DatePicker
                        width=InputWidth::Medium
                        label="Bis"
                        min=Some(day)
                        name="end_date"
                        value=end_date
                    />
                    <Switch label="Mit Zeiten" checked=has_time />
                    <Show when=move || *has_time.read()>
                        <TimePicker
                            width=InputWidth::Medium
                            label="Startzeit"
                            required=true
                            name="start_time"
                            value=start_time
                        />
                        <TimePicker
                            width=InputWidth::Medium
                            label="Endzeit"
                            required=true
                            name="end_time"
                            value=end_time
                        />
                    </Show>
                    <span class="cosmo-input__header">Benachrichtigungen</span>
                    <label class="cosmo-label" for="new-option">
                        Zeitpunkt
                    </label>
                    <div class="cosmo-input is--group">
                        <input
                            class="cosmo-input"
                            id="new-option"
                            type="datetime-local"
                            bind:value=new_notification
                            form="new_notification"
                        />
                        <button
                            type="submit"
                            class="cosmo-button is--primary is--addon"
                            form="new_notification"
                        >
                            <Icon width="1rem" height="1rem" icon=LuPlus />
                        </button>
                    </div>
                    {move || {
                        notifications
                            .get()
                            .into_iter()
                            .map(|when| {
                                view! {
                                    <div class="pandas-custom-fields__option is--new">
                                        <span>{when.format_localized("%A %e %B %Y, %R", Locale::de_DE).to_string()}</span>
                                        <button
                                            type="button"
                                            class="cosmo-button is--negative is--custom-fields is--edit"
                                            on:click=move |_| delete_notification.run(when)
                                        >
                                            <Icon width="1.25rem" height="1.25rem" icon=LuTrash />
                                        </button>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </ModalContent>
                <ModalButton label="Abbrechen" on_click=move || is_open.set(false) slot />
                <ModalButton
                    variant=Variant::Negative
                    label="Event löschen"
                    on_click=delete_event
                    slot
                />
                <ModalButton label="Event speichern" is_submit=true slot />
            </FormModal>
        </Show>
    }
}

#[component]
pub fn Calendar(
    #[prop(optional, into)] grove_id: Signal<Option<i32>>,
    #[prop(optional, into)] grove_name: Signal<String>,
) -> impl IntoView {
    let query = use_query_map();

    let date = Memo::new(move |_| {
        let today = Local::now().date_naive().with_day(1).unwrap();
        if let Some(month) = query.get().get("month") {
            NaiveDate::parse_from_str(month.as_str(), "%Y-%m-%d").unwrap_or(today)
        } else {
            today
        }
    });

    let prev_month = Memo::new(move |_| date.read().sub(Months::new(1)));
    let current_month = Memo::new(move |_| date.read().month());
    let next_month = Memo::new(move |_| date.read().add(Months::new(1)));

    let first_day_of_month = date;
    let last_day_of_month = Memo::new(move |_| date.read().add(Months::new(1)).sub(Days::new(1)));

    let calendar_start_date = Memo::new(move |_| {
        let date = first_day_of_month.read();

        if date.weekday() == Weekday::Mon {
            *date
        } else {
            let offset = date.weekday() as u64 - 1;
            let last_day_of_prev_month = date.checked_sub_days(Days::new(1)).unwrap();

            let offset_days = Days::new(offset);
            last_day_of_prev_month
                .checked_sub_days(offset_days)
                .unwrap()
        }
    });
    let calendar_end_date = Memo::new(move |_| {
        let first_day = first_day_of_month;
        let date = last_day_of_month.read();

        let first_day_offset = if first_day.read().weekday() == Weekday::Sun {
            5
        } else {
            first_day.read().weekday() as i8 - 1
        };

        let days_of_next_month = 40 - first_day_offset - date.day() as i8;

        first_day
            .read()
            .checked_add_months(Months::new(1))
            .unwrap()
            .checked_add_days(Days::new(days_of_next_month as u64))
            .unwrap()
    });

    let calendar_range = Memo::new(move |_| {
        DateRange::new(calendar_start_date.get(), calendar_end_date.get())
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
    });

    let events = RwSignal::new(Vec::<GroveEvent>::default());
    let events_resource = Resource::new(
        move || {
            (
                calendar_end_date.get(),
                calendar_start_date.get(),
                grove_id.get(),
            )
        },
        |(end, start, grove_id)| async move { get_events(start, end, grove_id).await },
    );

    let base_url = Memo::new(move |_| {
        if let Some(id) = grove_id.get() {
            format!("/pandas/groves/{id}/{}", grove_name.get())
        } else {
            "/pandas/bamboo".to_string()
        }
    });

    let prev_href = Memo::new(move |_| format!("{}?month={}", base_url.get(), prev_month.get()));
    let next_href = Memo::new(move |_| format!("{}?month={}", base_url.get(), next_month.get()));

    let leptos_use::UseEventSourceReturn {
        message: created_event,
        ..
    } = leptos_use::use_event_source_with_options::<GroveEvent, codee::string::JsonSerdeCodec>(
        "/sse/event",
        leptos_use::UseEventSourceOptions::default()
            .named_events(vec![EventType::Created.to_string()]),
    );
    let leptos_use::UseEventSourceReturn {
        message: updated_event,
        ..
    } = leptos_use::use_event_source_with_options::<GroveEvent, codee::string::JsonSerdeCodec>(
        "/sse/event",
        leptos_use::UseEventSourceOptions::default()
            .named_events(vec![EventType::Updated.to_string()]),
    );
    let leptos_use::UseEventSourceReturn {
        message: deleted_event,
        ..
    } = leptos_use::use_event_source_with_options::<GroveEvent, codee::string::JsonSerdeCodec>(
        "/sse/event",
        leptos_use::UseEventSourceOptions::default()
            .named_events(vec![EventType::Deleted.to_string()]),
    );

    let _ = Effect::watch(
        move || created_event.get(),
        move |data, _, _| {
            if let Some(data) = data {
                events.update(|events| events.push(data.data.to_owned()));
            }
        },
        false,
    );
    let _ = Effect::watch(
        move || updated_event.get(),
        move |data, _, _| {
            if let Some(data) = data {
                events.update(|events| {
                    let event = data.data.to_owned();
                    if let Some(evt) = events.iter_mut().find(|evt| evt.id == event.id) {
                        *evt = event;
                    }
                });
            }
        },
        false,
    );
    let _ = Effect::watch(
        move || deleted_event.get(),
        move |data, _, _| {
            if let Some(data) = data {
                events.update(|events| {
                    let event = data.data.to_owned();
                    events.retain(|evt| evt.id != event.id)
                });
            }
        },
        false,
    );

    Effect::new(move || {
        events_resource.refetch();
    });

    view! {
        <Transition>
            {move || Suspend::new(async move {
                events_resource
                    .await
                    .map_or_else(|_| events.set(vec![]), move |evts| events.set(evts.clone()));
            })} <div class="pandas-calendar">
                <div class="pandas-calendar__header">
                    <A href=move || prev_href.get() attr:class="pandas-calendar__action is--prev">
                        <>
                            {move || {
                                prev_month
                                    .read()
                                    .format_localized("%B %Y", Locale::de_DE)
                                    .to_string()
                            }}
                        </>
                    </A>
                    <h2>
                        {move || date.read().format_localized("%B %Y", Locale::de_DE).to_string()}
                    </h2>
                    <A href=move || next_href.get() attr:class="pandas-calendar__action is--next">
                        <>
                            {move || {
                                next_month
                                    .read()
                                    .format_localized("%B %Y", Locale::de_DE)
                                    .to_string()
                            }}
                        </>
                    </A>
                </div>
                <div class="pandas-calendar__container">
                    <div class="pandas-calendar__weekday">Montag</div>
                    <div class="pandas-calendar__weekday">Dienstag</div>
                    <div class="pandas-calendar__weekday">Mittwoch</div>
                    <div class="pandas-calendar__weekday">Donnerstag</div>
                    <div class="pandas-calendar__weekday">Freitag</div>
                    <div class="pandas-calendar__weekday">Samstag</div>
                    <div class="pandas-calendar__weekday">Sonntag</div>
                    <For
                        each=move || calendar_range.get()
                        key=move |day| format!("{day}{}", current_month.read())
                        let(day)
                    >
                        {
                            let events_for_day = Memo::new(move |_| {
                                events
                                    .read()
                                    .iter()
                                    .filter(move |event| {
                                        event.start_date <= day && event.end_date >= day
                                    })
                                    .cloned()
                                    .collect::<Vec<_>>()
                            });
                            view! {
                                <Day
                                    grove_id=grove_id
                                    events=events_for_day
                                    day=day.day()
                                    month=day.month()
                                    year=day.year()
                                    selected_month=current_month
                                />
                            }
                        }
                    </For>
                </div>
            </div>
        </Transition>
    }
}
