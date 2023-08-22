use async_graphql::MergedObject;

pub mod sign_up;
pub mod verify_email;

use sign_up::AddSignUpMutation;
use verify_email::VerifyMutation;

#[derive(MergedObject, Default)]
pub struct UserMutation(pub AddSignUpMutation, pub VerifyMutation);
