pub mod exercise;
pub mod exercise_equipment;
pub mod exercise_instruction;
pub mod exercise_muscle_map;
mod errors;
pub mod link;
pub mod muscle;
pub mod muscle_group;
pub mod user;
pub mod profile;

pub use exercise::Exercise;
pub use exercise_equipment::ExerciseEquipment;
pub use exercise_instruction::ExerciseInstruction;
pub use exercise_muscle_map::ExerciseMuscleMap;
pub use errors::Error;
pub use link::Link;
pub use muscle::Muscle;
pub use muscle_group::MuscleGroup;
pub use profile::Profile;
pub use user::User;

pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;
