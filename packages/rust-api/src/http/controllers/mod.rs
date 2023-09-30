mod auth;
mod dev;
mod link;
mod muscle;

pub use auth::AuthController;
pub use dev::DevController;
pub use link::LinkController;
pub use muscle::MuscleController;

use axum::Router;

pub trait Controller {
    type State;

    fn router(state: Self::State) -> Router;
}
