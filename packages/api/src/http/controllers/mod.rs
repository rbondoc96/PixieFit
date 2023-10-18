mod auth;
mod dev;
mod errors;
mod exercise;
mod exercise_equipment;
mod link;
mod muscle;
mod muscle_group;

pub use auth::AuthController;
pub use dev::DevController;
pub use exercise::ExerciseController;
pub use exercise_equipment::ExerciseEquipmentController;
pub use link::LinkController;
pub use muscle::MuscleController;
pub use muscle_group::MuscleGroupController;
pub(self) use errors::Error;
pub(self) type Result<TValue> = ::core::result::Result<TValue, crate::error::Error>;

use axum::Router;

pub trait Controller {
    type State;

    fn router(state: Self::State) -> Router;
}
