use bamboo_common_core::entities;
use bamboo_common_core::entities::mail::{
    MAIL_STATUS_FAILED, MAIL_STATUS_OPEN, MAIL_STATUS_SENDING,
};
use bamboo_common_core::entities::Mail;
use sea_orm::prelude::*;
use sea_orm::IntoActiveModel;
use std::error::Error;

pub async fn enqueue_mail(mail: Mail, db: &DatabaseConnection) {
    let res = entities::mail::Entity::insert(mail.into_active_model())
        .exec(db)
        .await;
    if let Err(err) = res {
        log::error!("Failed to enqueue mail {err}")
    }
}

pub async fn mark_sending(mail: &Mail, db: &DatabaseConnection) {
    let res = entities::mail::Entity::update_many()
        .col_expr(
            entities::mail::Column::Status,
            Expr::value(MAIL_STATUS_SENDING),
        )
        .filter(entities::mail::Column::Id.eq(mail.id))
        .exec(db)
        .await;
    if let Err(err) = res {
        log::error!("Failed to mark mail sending {err}")
    }
}

pub async fn mark_failed(mail: &Mail, err: &impl Error, db: &DatabaseConnection) {
    let res = entities::mail::Entity::update_many()
        .col_expr(
            entities::mail::Column::Status,
            Expr::value(MAIL_STATUS_FAILED),
        )
        .col_expr(entities::mail::Column::Error, Expr::value(err.to_string()))
        .filter(entities::mail::Column::Id.eq(mail.id))
        .exec(db)
        .await;
    if let Err(err) = res {
        log::error!("Failed to mark mail failed {err}")
    }
}

pub async fn mark_sent(mail: &Mail, db: &DatabaseConnection) {
    let res = entities::mail::Entity::delete_by_id(mail.id).exec(db).await;
    if let Err(err) = res {
        log::error!("Failed to delete mail {err}")
    }
}

pub async fn get_pending_mails(db: &DatabaseConnection) -> Result<Vec<Mail>, DbErr> {
    entities::mail::Entity::find()
        .filter(entities::mail::Column::Status.eq(MAIL_STATUS_OPEN))
        .all(db)
        .await
}
