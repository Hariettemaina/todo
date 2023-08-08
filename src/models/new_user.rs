use diesel::Insertable;
use uuid::Uuid;

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email_address: &'a str,
    pub email_verification_code: Uuid,
    pub email_verification_code_expiry: chrono::NaiveDateTime,
}