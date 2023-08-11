use async_graphql::MergedObject;

use self::add_category::AddCategoryMutation;
use self::add_categorylookup::AddCategoryLookupMutation;
use self::add_todo::AddTodoMutation;
use self::sign_up::AddSignUpMutation;

pub mod sign_up;
pub mod add_todo;
pub mod add_category;
pub mod add_categorylookup;

#[derive(MergedObject, Default)]
pub struct Mutation(
    pub AddSignUpMutation,
    pub AddTodoMutation,
    pub AddCategoryMutation,
    pub AddCategoryLookupMutation
);

