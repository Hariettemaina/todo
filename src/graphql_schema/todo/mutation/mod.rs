use async_graphql::MergedObject;

pub mod add_todo;

use add_todo::AddTodoMutation;

#[derive(MergedObject, Default)]
pub struct TodoMutation(pub AddTodoMutation);
