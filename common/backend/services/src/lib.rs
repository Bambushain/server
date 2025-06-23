use actix_web::web;
use sea_orm::DatabaseConnection;

pub use crate::environment_service::EnvironmentService;
pub use crate::minio_service::MinioClient;
pub use crate::totp_service::TotpService;

mod environment_service;
mod minio_service;
mod totp_service;

pub type EnvService = web::Data<EnvironmentService>;
pub type DbConnection = web::Data<DatabaseConnection>;
pub type MinioService = web::Data<MinioClient>;
