pub mod add_categorylookup;

use async_graphql::MergedObject;

use add_categorylookup::AddCategoryLookupMutation;

#[derive(MergedObject, Default)]
pub struct CategorysMutation(pub AddCategoryLookupMutation);
