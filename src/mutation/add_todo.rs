use async_graphql::{Context, InputObject, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use crate::{models::{NewToDo,ToDo},  ToDoError};
use super::Mutation;


#[derive(InputObject)]
pub struct ITodo{
    pub id: i32,
    pub username: String,
    pub title: String,
    pub completed: bool,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub completed_date: Option<chrono::NaiveDateTime>,
}
impl<'a> From<&'a ITodo> for NewToDo<'a> {
    fn from(input: &'a ITodo) -> Self {
        // map function is used to transform the Option<chrono::NaiveDate> values to Option<chrono::NaiveDateTime>. 
        //The NaiveDateTime is created by combining the date from NaiveDate with a default time of midnight (NaiveTime::from_hms(0, 0, 0)).
        let due_date = input.due_date.map(|date| {
            chrono::NaiveDateTime::new(date.into(), chrono::NaiveTime::from_hms_opt(0, 0, 0,).unwrap())
        });
//NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%d %H:%M:%S").unwrap()
        let completed_date = input.completed_date.map(|date| {
            chrono::NaiveDateTime::new(date.into(), chrono::NaiveTime::from_hms_opt(0, 0, 0,).unwrap())
        });

        Self {
            username: &input.username,
            title: &input.title,
            completed: input.completed,
            description: input.description.as_deref(),
            due_date,
            completed_date,
        }
    }
}

#[Object]    
impl Mutation {
    async fn add_todo<'ctx>(&self, ctx: &Context<'ctx>, credentials: ITodo) -> Result<bool> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let new_todo: NewToDo = (&credentials).into();

        diesel::insert_into(todo::table)
            .values(new_todo)
            .execute(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                ToDoError::UserAccountAlreadyExists
            })?;

        Ok(true)
    }
    
}



