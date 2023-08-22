use async_graphql::{Context, Object, Result};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::User;

#[derive(Default)]
pub struct VerifyMutation;

#[Object]
impl VerifyMutation {
    pub async fn verify_email(&self, ctx: &Context<'_>, code: String) -> Result<User> {
        use crate::schema::users::dsl::{email_verification_code, email_verified, users};

        let code = Uuid::parse_str(&code).unwrap();

        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        Ok(
            diesel::update(users.filter(email_verification_code.eq(code)))
                .set(email_verified.eq(true))
                .get_result::<User>(&mut connection)
                .await
                .map_err(|e| {
                    log::error!("Failed to verify email: {}", e);
                    e
                })?,
        )
    }
}
