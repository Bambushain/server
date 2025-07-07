use crate::api::{get_events, CreateEventAction, DeleteEventAction, UpdateEventAction};
use crate::state::AllGroves;
use bamboo_common::core::entities::{BambooUser, GroveEvent};
#[cfg(any(feature = "csr", feature = "hydrate"))]
use bamboo_common::core::queueing::EventType;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use leptos::prelude::*;
use leptos_cosmo::icons::Icon;
use leptos_cosmo::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Not, Sub};
#[cfg(any(feature = "csr", feature = "hydrate"))]
use std::str::FromStr;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use strum::IntoEnumIterator;

enum ColorYiqResult {
    Light,
    Dark,
}

impl Display for ColorYiqResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ColorYiqResult::Light => "#ffffff",
            ColorYiqResult::Dark => "#333333",
        })
    }
}

fn color_yiq(color: String) -> ColorYiqResult {
    let color = Color::from_hex(color.as_str()).unwrap();
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

    let color = Color::from_hex(event.color.as_str()).unwrap_or_default();

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
            style:--event-text-color=color_yiq(event.color.clone()).to_string()
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
fn AddEventDialog(
    day: NaiveDate,
    grove_id: Signal<Option<i32>>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let groves = expect_context::<RwSignal<AllGroves>>();

    let action = ServerAction::<CreateEventAction>::new();

    let start_date = RwSignal::new(day);
    let end_date = RwSignal::new(day);

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

    let color = RwSignal::new(Color::random());

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

    let value = action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    Effect::new(move |_| {
        if value.read().is_some() {
            is_open.set(false);
        }
    });

    view! {
        <ActionFormModal action=action title="Event hinzufügen">
            <ModalContent slot>
                <Show when=has_error>
                    <AlertMessage header="Fehler beim Hinzufügen">
                        <MessageContent slot>
                            Das Event konnte leider nicht hinzugefügt werden, bitte wende dich an den Bambussupport.
                        </MessageContent>
                    </AlertMessage>
                </Show>
                <Show when=move || current_grove_id.read().is_some()>
                    <input
                        type="hidden"
                        name="grove"
                        value=move || current_grove_id.get().unwrap().id
                    />
                </Show>
                <Textbox width=InputWidth::Medium label="Titel" required=true name="title" />
                <Textarea
                    width=InputWidth::Medium
                    label="Beschreibung"
                    name="description"
                    required=false
                />
                <ColorPicker width=InputWidth::Medium label="Farbe" name="color" value=color />
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
                    />
                    <TimePicker
                        width=InputWidth::Medium
                        label="Endzeit"
                        required=true
                        name="end_time"
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
            </ModalContent>
            <ModalButton label="Abbrechen" on_click=move || is_open.set(false) slot />
            <ModalButton label="Event speichern" is_submit=true slot />
        </ActionFormModal>
    }
}

#[component]
fn EditEventDialog(event: GroveEvent, is_open: RwSignal<bool>) -> impl IntoView {
    let update_action = ServerAction::<UpdateEventAction>::new();
    let delete_action = ServerAction::<DeleteEventAction>::new();

    let update_value = update_action.value();
    let delete_value = delete_action.value();

    let has_update_error = move || update_value.with(|val| matches!(val, Some(Err(_))));
    let has_delete_error = move || delete_value.with(|val| matches!(val, Some(Err(_))));

    Effect::new(move |_| {
        if update_value.read().is_some() {
            is_open.set(false);
        }
    });

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
    let color = RwSignal::new(
        Color::from_hex(event.color.as_str()).unwrap_or(Color::new(89, 140, 121, 0.0)),
    );

    let has_time = RwSignal::new(event.start_time.is_some());

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
            <ActionFormModal action=update_action title="Event bearbeiten">
                <ModalContent slot>
                    <Show when=has_update_error>
                        <AlertMessage header="Fehler beim Speichern">
                            <MessageContent slot>
                                Das Event konnte leider nicht gespeichert werden, bitte wende dich an den Bambussupport.
                            </MessageContent>
                        </AlertMessage>
                    </Show>
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
                    <ColorPicker width=InputWidth::Medium label="Farbe" name="color" value=color />
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
                </ModalContent>
                <ModalButton label="Abbrechen" on_click=move || is_open.set(false) slot />
                <ModalButton
                    variant=Variant::Negative
                    label="Event löschen"
                    on_click=delete_event
                    slot
                />
                <ModalButton label="Event speichern" is_submit=true slot />
            </ActionFormModal>
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

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let leptos_use::UseEventSourceReturn { event, data, .. } =
            leptos_use::use_event_source_with_options::<GroveEvent, codee::string::JsonSerdeCodec>(
                "/sse/event",
                leptos_use::UseEventSourceOptions::default().named_events(
                    EventType::iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<_>>(),
                ),
            );
        let _ = Effect::watch(
            move || data.get(),
            move |data, _, _| {
                if let Some(data) = data {
                    events.update(|events| {
                        if let Some(event) = event.get() {
                            let event_type = EventType::from_str(event.type_().as_str());
                            match event_type {
                                Ok(EventType::Created) => events.push(data.to_owned()),
                                Ok(EventType::Updated) => {
                                    if let Some(evt) =
                                        events.iter_mut().find(|evt| evt.id == data.id)
                                    {
                                        *evt = data.to_owned();
                                    }
                                }
                                Ok(EventType::Deleted) => events.retain(|evt| evt.id != data.id),
                                _ => {}
                            }
                        }
                    });
                }
            },
            false,
        );
    }

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
