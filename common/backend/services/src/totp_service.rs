use bamboo_common_backend_dbal as dbal;
use bamboo_common_core::entities::TotpQrCode;
use bamboo_common_core::entities::user::BambooUser;
use bamboo_common_core::error::BambooError;
use base64::engine::Engine;
use fast_qr::QRBuilder;
use fast_qr::convert::Builder;
use fast_qr::convert::Shape;
use fast_qr::convert::svg::SvgBuilder;
use sea_orm::DatabaseConnection;

pub struct TotpService {}

impl Default for TotpService {
    fn default() -> Self {
        Self::new()
    }
}

impl TotpService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_totp_qr(
        &self,
        user: BambooUser,
        db: &DatabaseConnection,
    ) -> Result<TotpQrCode, BambooError> {
        let totp = totp_rs::Builder::new()
            .with_account_name(user.display_name)
            .with_issuer(Some("Bambushain"))
            .build()
            .map_err(|_| BambooError::crypto("TotpService", "Failed to build totp"))?;
        let secret = totp.secret().clone().to_vec();
        dbal::enable_my_totp(user.id, secret, db).await.map(|_| {
            let totp_url = totp.to_url().unwrap_or_default();
            let qr = QRBuilder::new(totp_url).build().map_err(|err| {
                log::error!("Failed to enable totp {err}");
                let db = db.clone();
                actix_web::rt::spawn(async move {
                    let db = db.clone();

                    let _ = dbal::disable_my_totp(user.id, &db).await;
                });

                BambooError::unknown("user", "Failed to create qr code")
            })?;
            let qr_svg = SvgBuilder::default()
                .shape(Shape::Circle)
                .background_color("transparent")
                .module_color("#598c79")
                .to_str(&qr);
            let qr_svg_data_url = format!(
                "data:image/svg+xml;base64,{}",
                base64::prelude::BASE64_STANDARD.encode(qr_svg)
            );

            Ok(TotpQrCode {
                qr_code: qr_svg_data_url,
                secret: totp.secret().to_base32(),
            })
        })?
    }
}
