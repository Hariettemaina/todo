use async_graphql::SimpleObject;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(SimpleObject,Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub display_name: Option<String>,
    pub password: String,
    pub email_address: String,
    pub email_verified: bool,
    pub email_verification_code: Uuid,
    pub email_verification_code_expiry: chrono::NaiveDateTime,
}