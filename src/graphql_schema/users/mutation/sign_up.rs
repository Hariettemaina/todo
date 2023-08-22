use async_graphql::{Context, InputObject, Object, Result};

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    models::NewUser,
    password::PassWordHasher,
    schema::users::{self},
    ToDoError,
};

#[derive(InputObject)]
pub struct ISignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl<'a> From<&'a ISignUp> for NewUser<'a> {
    fn from(input: &'a ISignUp) -> Self {
        Self {
            username: &input.username,
            password: &input.password,
            email_address: &input.email,
            email_verification_code: Uuid::new_v4(),
            email_verification_code_expiry: chrono::Local::now()
                .naive_local()
                .checked_add_signed(chrono::Duration::hours(24))
                .unwrap(),
        }
    }
}
#[derive(Default)]
pub struct AddSignUpMutation;
#[Object]
impl AddSignUpMutation {
    pub async fn sign_up<'ctx>(&self, ctx: &Context<'ctx>, credentials: ISignUp) -> Result<bool> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;
        let hasher = ctx
            .data::<PassWordHasher>()
            .map_err(|e| {
                log::error!("Failed to get app data: {:?}", e);
                e
            })
            .unwrap();

            let password = hasher.hash_password(credentials.password.clone()).unwrap();

            let new_user: NewUser = (&credentials).into();
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                ToDoError::UserAccountAlreadyExists
            })?;

        Ok(true)
    }
}
