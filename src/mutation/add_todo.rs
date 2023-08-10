use async_graphql::{Context, InputObject, Object, Result};
use chrono::NaiveDate;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use crate::{models::NewToDo,  ToDoError, schema::todos};


#[derive(InputObject)]
pub struct ITodo {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub completed: bool,
    pub description: Option<String>,
    pub due_date: Option<String>, 
    pub completed_date: Option<String>, 
}
impl<'a> From<&'a ITodo> for NewToDo<'a>{
    fn from(input: &'a ITodo) -> Self {
        let due_date = input.due_date.clone().and_then(|date| {
            NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()
        });

        let completed_date = input.completed_date.clone().and_then(|date| {
            NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()
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
pub struct AddTodoMutation;
#[Object]    
impl AddTodoMutation {
    async fn add_todo<'ctx>(&self, ctx: &Context<'ctx>, credentials: ITodo) -> Result<bool> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let new_todo: NewToDo = (&credentials).into();

        diesel::insert_into(todos::table)
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



