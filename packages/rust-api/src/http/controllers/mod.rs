mod auth;
mod dev;
mod link;
mod muscle;
mod muscle_group;

pub use auth::AuthController;
pub use dev::DevController;
pub use link::LinkController;
pub use muscle::MuscleController;
pub use muscle_group::MuscleGroupController;

use axum::Router;

pub trait Controller {
    type State;

    fn router(state: Self::State) -> Router;
}
