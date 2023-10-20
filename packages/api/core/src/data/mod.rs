use crate::enums::{
    ExerciseForce,
    ExerciseMechanic,
    ExerciseMuscleTarget,
    ExerciseType,
    Gender,
    Measurement,
    Role,
};
use serde::Deserialize;

#[derive(Debug)]
pub struct CreateUserData<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub role: Option<Role>,
    pub password: &'a str,
    pub password_confirm: &'a str,
}

#[derive(Debug)]
pub struct CreateUserProfileData {
    pub user_id: i64,
    pub birthday: chrono::NaiveDate,
    pub gender: Gender,
}
