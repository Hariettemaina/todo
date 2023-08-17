use crate::{models::{User, NewUser}, password::PassWordHasher, schema::users, ToDoError};
use async_graphql::{Context, Object, Result, SimpleObject};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{ AsyncPgConnection, pooled_connection::deadpool::Pool};
use uuid::Uuid;
use diesel_async::RunQueryDsl;



#[derive(SimpleObject)]
pub struct Ilogin {
    pub username:String,
    pub email: String,
    pub password: String,
}
impl<'a> From<&'a Ilogin> for NewUser<'a> {
    fn from(input: &'a Ilogin) -> Self {
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
pub struct LoginQuery;
#[Object]
impl LoginQuery {

    pub async fn login<'ctx>(&self, ctx: &Context<'ctx>, email: String, pass:String) -> Result<User> {
        

        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;
        
        let hasher = ctx
            .data::<PassWordHasher>()
            .map_err(|e| {
                log::error!("Failed to get app data: {:?}", e);
                e
            }).unwrap();
        let user = users::table
            .filter(users::email_address.eq(&email))
            .filter(users::password.eq(&pass))
            .first::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to fetch user: {}", e);
                ToDoError::UserNotFound
            })?;
            
            if let false = hasher.verify_password(pass, user.password.clone()) {
                return Err(async_graphql::Error::new(ToDoError::InvalidCredentials.to_string()));
            }
            Ok(user)
        }

        
    }
