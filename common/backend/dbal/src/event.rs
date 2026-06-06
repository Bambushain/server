use crate::error_tag;
use bamboo_common_core::entities::event;
use bamboo_common_core::entities::event::{GroveEvent, GroveEventReminder};
use bamboo_common_core::entities::user::WebUser;
use bamboo_common_core::entities::*;
use bamboo_common_core::error::*;
use bamboo_common_core::queueing::EventAction;
use chrono::{NaiveDate, Timelike, Utc};
use date_range::DateRange;
use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{
    Condition, FromQueryResult, IntoActiveModel, JoinType, NotSet, QueryOrder, QuerySelect,
    SelectModel, Selector, Set, TransactionTrait,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default, FromQueryResult)]
struct LoadEvent {
    pub event_id: i32,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub color: String,
    pub is_private: bool,
    pub user_id: Option<i32>,
    pub display_name: Option<String>,
    pub discord_name: Option<String>,
    pub email: Option<String>,
    pub grove_id: Option<i32>,
    pub grove_name: Option<String>,
}

impl LoadEvent {
    fn into_grove_event(self, notifications: Vec<EventReminder>) -> GroveEvent {
        let user = self.user_id.map(|id| WebUser {
            id,
            email: self.email.unwrap(),
            display_name: self.display_name.unwrap(),
            discord_name: self.discord_name.unwrap_or("".to_string()),
        });
        let grove = self.grove_id.map(|id| Grove {
            id,
            name: self.grove_name.unwrap(),
            invite_secret: None,
        });

        GroveEvent {
            id: self.event_id,
            title: self.title,
            description: self.description,
            start_date: self.start_date,
            end_date: self.end_date,
            start_time: self.start_time,
            end_time: self.end_time,
            color: self.color,
            is_private: self.is_private,
            user,
            grove,
            reminder: notifications
                .into_iter()
                .map(|notification| GroveEventReminder {
                    id: notification.id,
                    when: notification.time,
                })
                .collect_vec(),
        }
    }
}

fn get_event_query(
    user_id: i32,
    additional_filter: impl IntoCondition,
) -> Selector<SelectModel<LoadEvent>> {
    event::Entity::find()
        .select_only()
        .column_as(event::Column::Id, "event_id")
        .column_as(event::Column::Title, "title")
        .column_as(event::Column::Description, "description")
        .column_as(event::Column::StartDate, "start_date")
        .column_as(event::Column::EndDate, "end_date")
        .column_as(event::Column::StartTime, "start_time")
        .column_as(event::Column::EndTime, "end_time")
        .column_as(event::Column::Color, "color")
        .column_as(event::Column::IsPrivate, "is_private")
        .column_as(user::Column::Id, "user_id")
        .column_as(user::Column::DisplayName, "display_name")
        .column_as(user::Column::DiscordName, "discord_name")
        .column_as(user::Column::Email, "email")
        .column_as(grove::Column::Id, "grove_id")
        .column_as(grove::Column::Name, "grove_name")
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(event::Entity)
                .from(grove_user::Column::GroveId)
                .to(event::Column::GroveId)
                .on_condition(|gu, e| {
                    Condition::all()
                        .add(
                            Expr::col((gu.clone(), grove_user::Column::GroveId))
                                .eq(Expr::col((e.clone(), event::Column::GroveId))),
                        )
                        .add(
                            Condition::any()
                                .add(
                                    Expr::col((gu, grove_user::Column::UserId))
                                        .eq(Expr::col((e.clone(), event::Column::UserId))),
                                )
                                .add(Expr::col((e, event::Column::UserId)).is_null()),
                        )
                })
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            grove::Entity::belongs_to(event::Entity)
                .from(grove::Column::Id)
                .to(event::Column::GroveId)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            user::Entity::belongs_to(event::Entity)
                .from(user::Column::Id)
                .to(event::Column::UserId)
                .into(),
        )
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(true))
                        .add(event::Column::UserId.eq(user_id)),
                )
                .add(
                    Condition::all()
                        .add(event::Column::IsPrivate.eq(false))
                        .add(grove_user::Column::UserId.eq(user_id)),
                ),
        )
        .filter(additional_filter)
        .order_by_asc(event::Column::Id)
        .into_model::<LoadEvent>()
}

pub async fn get_events(
    range: DateRange,
    user_id: i32,
    grove_id: Option<i32>,
    db: &DatabaseConnection,
) -> BambooResult<Vec<GroveEvent>> {
    let additional_filter = Condition::any()
        .add(event::Column::StartDate.between(range.since(), range.until()))
        .add(event::Column::EndDate.between(range.since(), range.until()));

    let events = get_event_query(
        user_id,
        if let Some(grove_id) = grove_id {
            Condition::all()
                .add(additional_filter)
                .add(event::Column::GroveId.eq(grove_id))
        } else {
            additional_filter
        },
    )
    .all(db)
    .await
    .map_err(|_| BambooError::database(error_tag!(), "Failed to load events"))?;

    let mut result = vec![];
    for event in events {
        let notifications = get_event_reminder_by_event(event.event_id, db).await?;
        result.push(event.into_grove_event(notifications));
    }

    Ok(result)
}

pub async fn get_event(id: i32, user_id: i32, db: &DatabaseConnection) -> BambooResult<GroveEvent> {
    let notifications = get_event_reminder_by_event(id, db).await?;

    get_event_query(user_id, Condition::all().add(event::Column::Id.eq(id)))
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load events"))
        .map(|data| {
            data.map_or_else(
                || {
                    Err(BambooError::not_found(
                        error_tag!(),
                        "The event was not found",
                    ))
                },
                |data| Ok(data.into_grove_event(notifications)),
            )
        })?
}

pub async fn get_event_by_id(event_id: i32, db: &DatabaseConnection) -> BambooResult<GroveEvent> {
    let notifications = get_event_reminder_by_event(event_id, db).await?;

    event::Entity::find()
        .select_only()
        .column_as(event::Column::Id, "event_id")
        .column_as(event::Column::Title, "title")
        .column_as(event::Column::Description, "description")
        .column_as(event::Column::StartDate, "start_date")
        .column_as(event::Column::EndDate, "end_date")
        .column_as(event::Column::StartTime, "start_time")
        .column_as(event::Column::EndTime, "end_time")
        .column_as(event::Column::Color, "color")
        .column_as(event::Column::IsPrivate, "is_private")
        .column_as(user::Column::Id, "user_id")
        .column_as(user::Column::DisplayName, "display_name")
        .column_as(user::Column::DiscordName, "discord_name")
        .column_as(user::Column::Email, "email")
        .column_as(grove::Column::Id, "grove_id")
        .column_as(grove::Column::Name, "grove_name")
        .join_rev(
            JoinType::LeftJoin,
            grove_user::Entity::belongs_to(event::Entity)
                .from(grove_user::Column::GroveId)
                .to(event::Column::GroveId)
                .on_condition(|gu, e| {
                    Condition::all()
                        .add(
                            Expr::col((gu.clone(), grove_user::Column::GroveId))
                                .eq(Expr::col((e.clone(), event::Column::GroveId))),
                        )
                        .add(
                            Condition::any()
                                .add(
                                    Expr::col((gu, grove_user::Column::UserId))
                                        .eq(Expr::col((e.clone(), event::Column::UserId))),
                                )
                                .add(Expr::col((e, event::Column::UserId)).is_null()),
                        )
                })
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            grove::Entity::belongs_to(event::Entity)
                .from(grove::Column::Id)
                .to(event::Column::GroveId)
                .into(),
        )
        .join_rev(
            JoinType::LeftJoin,
            user::Entity::belongs_to(event::Entity)
                .from(user::Column::Id)
                .to(event::Column::UserId)
                .into(),
        )
        .filter(event::Column::Id.eq(event_id))
        .order_by_asc(event::Column::Id)
        .into_model::<LoadEvent>()
        .one(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to load events"))
        .map(|data| {
            data.map_or_else(
                || {
                    Err(BambooError::not_found(
                        error_tag!(),
                        "The event was not found",
                    ))
                },
                |data| Ok(data.into_grove_event(notifications)),
            )
        })?
}

pub async fn create_event(
    event: GroveEvent,
    user_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<GroveEvent> {
    let mut model = event.to_event().into_active_model();
    model.id = NotSet;
    model.user_id = Set(Some(user_id));

    let tx = db
        .begin()
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create event"))?;

    let created = model
        .insert(&tx)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create event"))?;

    event_reminder::Entity::delete_many()
        .filter(event_reminder::Column::EventId.eq(created.id))
        .exec(&tx)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create event"))?;

    event_reminder::Entity::insert_many(event.reminder.into_iter().map(|notification| {
        event_reminder::ActiveModel {
            id: NotSet,
            event_id: Set(created.id),
            time: Set(notification.when),
            notified: Set(notification.when < Utc::now()),
        }
    }))
    .exec(&tx)
    .await
    .map_err(|_| BambooError::database(error_tag!(), "Failed to create event"))?;

    tx.commit()
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create event"))?;

    let event = get_event(created.id, user_id, db).await?;
    bamboo_common_backend_notification::enqueue_event(EventAction::Created(event.clone())).await;

    Ok(event)
}

pub async fn update_event(
    user_id: i32,
    id: i32,
    event: GroveEvent,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    let _ = get_event(id, user_id, db).await?;

    let tx = db
        .begin()
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update event"))?;

    event::Entity::update_many()
        .filter(event::Column::Id.eq(id))
        .col_expr(event::Column::StartDate, Expr::value(event.start_date))
        .col_expr(event::Column::EndDate, Expr::value(event.end_date))
        .col_expr(event::Column::StartTime, Expr::value(event.start_time))
        .col_expr(event::Column::EndTime, Expr::value(event.end_time))
        .col_expr(event::Column::Description, Expr::value(event.description))
        .col_expr(event::Column::Title, Expr::value(event.title))
        .col_expr(event::Column::Color, Expr::value(event.color))
        .exec(&tx)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update event"))?;

    event_reminder::Entity::delete_many()
        .filter(event_reminder::Column::EventId.eq(id))
        .exec(&tx)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update event"))?;

    event_reminder::Entity::insert_many(event.reminder.into_iter().map(|notification| {
        event_reminder::ActiveModel {
            id: NotSet,
            event_id: Set(id),
            time: Set(notification.when),
            notified: Set(notification.when < Utc::now()),
        }
    }))
    .exec(&tx)
    .await
    .map_err(|_| BambooError::database(error_tag!(), "Failed to update event"))?;

    tx.commit()
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update event"))?;

    if let Ok(event) = get_event(id, user_id, db).await {
        bamboo_common_backend_notification::enqueue_event(EventAction::Updated(event.clone()))
            .await;
    }

    Ok(())
}

pub async fn delete_event(user_id: i32, id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    let event = get_event(id, user_id, db).await?;

    let res = event::Entity::delete_many()
        .filter(event::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete event"))?;

    if res.rows_affected > 0 {
        bamboo_common_backend_notification::enqueue_event(EventAction::Deleted(event.clone()))
            .await;

        Ok(())
    } else {
        Err(BambooError::not_found(error_tag!(), "Event not found"))
    }
}

pub async fn get_event_reminder_by_event(
    event_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<EventReminder>> {
    event_reminder::Entity::find()
        .filter(event_reminder::Column::EventId.eq(event_id))
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to get event reminder"))
}

pub async fn get_current_and_past_event_reminder(
    db: &DatabaseConnection,
) -> BambooResult<Vec<EventReminder>> {
    let now = Utc::now()
        .with_nanosecond(0)
        .unwrap()
        .with_second(0)
        .unwrap();
    event_reminder::Entity::find()
        .filter(event_reminder::Column::Time.lte(now))
        .filter(event_reminder::Column::Notified.eq(false))
        .all(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to get event reminder"))
}

pub async fn mark_event_reminder_notified(event_id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    event_reminder::Entity::update_many()
        .filter(event_reminder::Column::EventId.eq(event_id))
        .col_expr(event_reminder::Column::Notified, Expr::value(true))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to update event reminder"))
        .map(|_| ())
}

pub async fn create_event_reminder(
    event_id: i32,
    time: chrono::DateTime<chrono::Utc>,
    db: &DatabaseConnection,
) -> BambooResult<EventReminder> {
    let mut model = event_reminder::ActiveModel::new();
    model.id = NotSet;
    model.event_id = Set(event_id);
    model.time = Set(time);

    model
        .insert(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to create event reminder"))
}

pub async fn delete_event_reminder(
    event_id: i32,
    reminder_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    event_reminder::Entity::delete_many()
        .filter(event_reminder::Column::Id.eq(reminder_id))
        .filter(event_reminder::Column::EventId.eq(event_id))
        .exec(db)
        .await
        .map_err(|_| BambooError::database(error_tag!(), "Failed to delete event reminder"))
        .map(|_| ())
}
