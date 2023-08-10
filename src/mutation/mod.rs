use async_graphql::MergedObject;

use self::add_todo::AddTodoMutation;
use self::sign_up::AddSignUpMutation;
pub mod sign_up;
mod login;
pub mod add_todo;
#[derive(MergedObject)]
pub struct Mutation(pub AddSignUpMutation, pub AddTodoMutation);

