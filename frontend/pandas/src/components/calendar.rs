use crate::api::{get_events, CreateEventAction, DeleteEventAction, UpdateEventAction};
use bamboo_common::core::entities::{Grove, GroveEvent, User};
use bamboo_common::core::queueing::EventType;
use chrono::prelude::*;
use chrono::{Days, Months};
use date_range::DateRange;
use leptos::*;
use leptos_cosmo::icons::Icon;
use leptos_cosmo::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
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

#[derive(Clone, PartialEq)]
struct LoadEventData {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub grove_id: Option<i32>,
}

#[component]
fn EventEntry(event: GroveEvent) -> impl IntoView {
    let edit_event_open = create_rw_signal(false);

    let color = Color::from_hex(event.color.as_str()).unwrap_or(Color::default());

    let me = expect_context::<RwSignal<User>>();

    view! {
        <div
            class="pandas-calendar__event"
            style=format!("--event-background-color: {}; --event-shadow-color: {}; --event-text-color: {};", color.fade(0.8).hsla().clone(), color.fade(0.9).hsla().clone(), color_yiq(event.color.clone()))
        >
            { event.title.clone() }
            <Show when={
                let event = event.clone();

                move || me.get().id == event.user.clone().map(|user| user.id).unwrap_or(-1)
            }>
                <Icon
                    icon={icondata_lu::LuPencil}
                    width="1rem"
                    height="1rem"
                    class="pandas-calendar__event-edit"
                    on:click=move |_| edit_event_open.set(true)
                />
            </Show>
            <div class="pandas-calendar__event-description">
                <hgroup class="pandas-calendar__event-header">
                    <h3>{event.title.clone()}</h3>
                    <h5>
                        {if let Some(grove) = event.grove.clone() {
                            {grove.name}
                        } else {
                            "Privates Event".to_string()
                        }}
                    </h5>
                </hgroup>
                <p>{ event.description.clone() }</p>
                <span class="panda-calendar__event-arrow" />
            </div>
        </div>
        <Show when={
            let edit_event_open = edit_event_open.clone();

            move || edit_event_open.get()
        }>
            <EditEventDialog event=event.clone() is_open=edit_event_open />
        </Show>
    }
}

#[component]
fn Day(
    day: u32,
    month: u32,
    year: i32,
    #[prop(into)] selected_month: MaybeSignal<u32>,
    events: Vec<GroveEvent>,
    #[prop(into)] grove_id: MaybeSignal<Option<i32>>,
) -> impl IntoView {
    let add_event_open = create_rw_signal(false);

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
        <div class="pandas-calendar__day" style=format!("--day-number-color: {day_number_color}; --text: '{day}'; --background-color: {background_color}")>
            <Icon
                icon={icondata_lu::LuCalendarPlus}
                height="1.5rem"
                width="1.5rem"
                class="pandas-calendar__event-add"
                on:click={
                    let add_event_open = add_event_open.clone();

                    move |_| add_event_open.set(true)
                }
            />
            <Show when=move || add_event_open.get()>
                <AddEventDialog day=NaiveDate::from_ymd_opt(year, month, day).unwrap() grove_id=grove_id is_open=add_event_open />
            </Show>
            {events.iter().map(move |evt| view! {
                <EventEntry event={evt.clone()} />
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn AddEventDialog(
    day: NaiveDate,
    grove_id: MaybeSignal<Option<i32>>,
    is_open: RwSignal<bool>,
) -> impl IntoView {
    let groves = expect_context::<RwSignal<Vec<Grove>>>();

    let action = create_server_action::<CreateEventAction>();

    let start_date = create_rw_signal(day);
    let end_date = create_rw_signal(day);

    let selected_grove = create_rw_signal(grove_id.get().map_or(
        groves.get().first().map(|grove| grove.id.to_string()),
        |id| {
            groves
                .get()
                .iter()
                .find(|grove| grove.id == id)
                .map(|grove| grove.name.clone())
        },
    ));

    let color = create_rw_signal(Color::random());

    let is_private = create_rw_signal(grove_id.get().is_none());

    let current_grove_id = {
        let groves = groves.clone();
        let selected_grove = selected_grove.clone();

        create_memo(move |_| {
            let selected_grove = groves.get().into_iter().find(|grove| {
                grove.name
                    == selected_grove
                        .get()
                        .map(|name| name.clone())
                        .unwrap_or("".to_string())
            });

            selected_grove
        })
    };

    let groves_options = move || {
        groves
            .get()
            .into_iter()
            .map(|grove| (Some(grove.id.to_string()), grove.name.clone()))
            .collect::<Vec<_>>()
    };

    let value = action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    create_effect(move |_| {
        if let Some(_) = value.get() {
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
                <Show when={
                    let current_grove_id = current_grove_id.clone();

                    move || current_grove_id.get().is_some()
                }>
                    <input type="hidden" name="grove" value=move || current_grove_id.get().unwrap().id />
                </Show>
                <Textbox
                    width=InputWidth::Medium
                    label="Titel"
                    required=true
                    name="title"
                />
                <Textarea
                    width=InputWidth::Medium
                    label="Beschreibung"
                    name="description"
                    required=false
                />
                <ColorPicker
                    width=InputWidth::Medium
                    label="Farbe"
                    name="color"
                    value=color
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
                <Show when={
                    let grove_id = grove_id.clone();

                    move || grove_id.get().is_none()
                }>
                    <Switch
                        label="Nur für mich"
                        checked=is_private
                        name="is_private"
                    />
                </Show>
                <Show when={
                    let grove_id = grove_id.clone();
                    let is_private = is_private.clone();

                    move || grove_id.get().is_none() && !is_private.get()
                }>
                    <SingleSelect
                        label="Hain"
                        required=true
                        items=groves_options()
                        name="grove"
                        selected=selected_grove.clone()
                    />
                </Show>
            </ModalContent>
            <ModalButton label="Abbrechen" on_click=move |_| is_open.set(false) slot />
            <ModalButton label="Event speichern" is_submit=true slot />
        </ActionFormModal>
    }
}

#[component]
fn EditEventDialog(event: GroveEvent, is_open: RwSignal<bool>) -> impl IntoView {
    let update_action = create_server_action::<UpdateEventAction>();
    let delete_action = create_server_action::<DeleteEventAction>();

    let update_value = update_action.value();
    let delete_value = delete_action.value();

    let has_update_error = move || update_value.with(|val| matches!(val, Some(Err(_))));
    let has_delete_error = move || delete_value.with(|val| matches!(val, Some(Err(_))));

    create_effect(move |_| {
        if update_value.get().is_some() {
            is_open.set(false);
        }
    });

    create_effect(move |_| {
        if delete_value.get().is_some() {
            is_open.set(false);
        }
    });

    let grove_id = event.grove.map(|g| g.id);
    let id = event.id;
    let day = event.start_date;

    let title = create_rw_signal(event.title);
    let description = create_rw_signal(event.description);
    let start_date = create_rw_signal(event.start_date);
    let end_date = create_rw_signal(event.end_date);
    let color = create_rw_signal(
        Color::from_hex(event.color.as_str()).unwrap_or(Color::new(89, 140, 121, 0.0)),
    );

    let delete_event = move |_| {
        confirm(
            "Event löschen",
            format!("Soll das Event {} wirklich gelöscht werden?", title.get()),
            Variant::Negative,
            "Event löschen",
            "Nicht löschen",
            Some(Callback::new(move |_| {
                delete_action.dispatch(DeleteEventAction { id })
            })),
            None,
        );
    };

    view! {
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
                <Textbox
                    width=InputWidth::Medium
                    label="Titel"
                    name="title"
                    value=title
                />
                <Textarea
                    width=InputWidth::Medium
                    label="Beschreibung"
                    name="description"
                    value=description
                    required=false
                />
                <ColorPicker
                    width=InputWidth::Medium
                    label="Farbe"
                    name="color"
                    value=color
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
            </ModalContent>
            <ModalButton label="Abbrechen" on_click=move |_| is_open.set(false) slot />
            <ModalButton variant=Variant::Negative label="Event löschen" on_click=delete_event slot />
            <ModalButton label="Event speichern" is_submit=true slot />
        </ActionFormModal>
    }
}

#[component]
pub fn Calendar(#[prop(optional, into)] grove_id: Option<i32>) -> impl IntoView {
    let date = RwSignal::new(Local::now().date_naive().with_day(1).unwrap());

    let prev_month = {
        let date = date.clone();

        create_memo(move |_| date.get() - Months::new(1))
    };
    let current_month = {
        let date = date.clone();

        create_memo(move |_| date.get().month())
    };
    let next_month = {
        let date = date.clone();

        create_memo(move |_| date.get() + Months::new(1))
    };

    let first_day_of_month = date.clone();
    let last_day_of_month = {
        let date = date.clone();

        create_memo(move |_| date.get() + Months::new(1) - Days::new(1))
    };

    let calendar_start_date = {
        let date = first_day_of_month.clone();

        create_memo(move |_| {
            let date = date.get();

            if date.weekday() == Weekday::Mon {
                date
            } else {
                let offset = date.weekday() as u64 - 1;
                let last_day_of_prev_month = date.checked_sub_days(Days::new(1)).unwrap();

                let offset_days = Days::new(offset);
                last_day_of_prev_month
                    .checked_sub_days(offset_days)
                    .unwrap()
            }
        })
    };
    let calendar_end_date = {
        let first_day = first_day_of_month.clone();
        let date = last_day_of_month.clone();

        create_memo(move |_| {
            let date = date.get();

            let first_day_offset = if first_day.get().weekday() == Weekday::Sun {
                5
            } else {
                first_day.get().weekday() as i8 - 1
            };

            let days_of_next_month = 40 - first_day_offset - date.day() as i8;

            first_day
                .get()
                .checked_add_months(Months::new(1))
                .unwrap()
                .checked_add_days(Days::new(days_of_next_month as u64))
                .unwrap()
        })
    };

    let load_event_data = create_memo(move |_| LoadEventData {
        end: calendar_end_date.get(),
        start: calendar_start_date.get(),
        grove_id,
    });

    let events = create_rw_signal(Vec::<GroveEvent>::default());
    let events_resource = create_resource(
        move || load_event_data.get(),
        |load_event_data| async move {
            get_events(
                load_event_data.start,
                load_event_data.end,
                load_event_data.grove_id,
            )
            .await
        },
    );

    let prev = {
        let date = date.clone();

        move |_| {
            date.update(|date| *date = date.checked_sub_months(Months::new(1)).unwrap());
            events_resource.refetch()
        }
    };
    let next = {
        let date = date.clone();

        move |_| {
            date.update(|date| *date = date.checked_add_months(Months::new(1)).unwrap());
            events_resource.refetch()
        }
    };

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
        let events = events.clone();
        let _ = watch(
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

    view! {
        <Transition>
            {move || {
                create_effect(move |_| {
                    events_resource.map(move |evts| events.set(evts.clone().unwrap_or(Default::default())));
                });
            }}
            <div class="pandas-calendar">
                <div class="pandas-calendar__header">
                    <span class="pandas-calendar__action is--prev">
                        <a on:click={prev}>{move || prev_month.get().format_localized("%B %Y", Locale::de_DE).to_string()}</a>
                    </span>
                    <h2>{move || date.get().format_localized("%B %Y", Locale::de_DE).to_string()}</h2>
                    <span class="pandas-calendar__action is--next">
                        <a on:click={next}>{move || next_month.get().format_localized("%B %Y", Locale::de_DE).to_string()}</a>
                    </span>
                </div>
                <div class="pandas-calendar__container">
                    <div class="pandas-calendar__weekday">Montag</div>
                    <div class="pandas-calendar__weekday">Dienstag</div>
                    <div class="pandas-calendar__weekday">Mittwoch</div>
                    <div class="pandas-calendar__weekday">Donnerstag</div>
                    <div class="pandas-calendar__weekday">Freitag</div>
                    <div class="pandas-calendar__weekday">Samstag</div>
                    <div class="pandas-calendar__weekday">Sonntag</div>
                        {move || DateRange::new(calendar_start_date.get(), calendar_end_date.get()).unwrap().into_iter().map(|day| {
                            let events_for_day = {
                                let events = events.clone();

                                move |day: NaiveDate| {
                                    events
                                        .get()
                                        .iter()
                                        .filter(move |event| event.start_date <= day && event.end_date >= day)
                                        .cloned()
                                        .collect::<Vec<_>>()
                                }
                            };

                            view! {
                                <Day
                                    grove_id={grove_id}
                                    events={events_for_day(day.clone())}
                                    day={day.day()}
                                    month={day.month()}
                                    year={day.year()}
                                    selected_month={current_month.get()}
                                />
                            }
                        }).collect::<Vec<_>>()}
                </div>
            </div>
        </Transition>
    }
}
