use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum Measurement {
    ExerciseDuration,
    ExerciseWeight,
}

impl Measurement {
    pub fn unit(&self) -> &'static str {
        match self {
            Self::ExerciseDuration => "seconds",
            Self::ExerciseWeight => "kilograms",
        }
    }

    pub fn denominator(&self) -> Option<&'static str> {
        match self {
            Self::ExerciseDuration => None,
            Self::ExerciseWeight => Some("reps"),
        }
    }

    pub fn operation(&self) -> &'static str {
        match self {
            Self::ExerciseDuration => "summation",
            Self::ExerciseWeight => "multiplication",
        }
    }
}
