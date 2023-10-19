use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum Measurement {
    Bodyweight,
    Duration,
    Repetitions,
    WeightedRepetitions,
    WeightedDuration,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
pub enum MeasurementUnit {
    Kilogram,
    Repetition,
    Second,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementDenominator {
    Repetition,
    Second,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementOperation {
    Addition,
    Division,
    Multiplication,
}

impl Measurement {
    pub fn unit(&self) -> &'static str {
        match self {
            Self::Bodyweight
                | Self::WeightedDuration
                | Self::WeightedRepetitions => "kilograms",
            Self::Duration => "seconds",
            Self::Repetitions => "repetitions",
        }
    }

    pub fn denominator(&self) -> Option<MeasurementDenominator> {
        match self {
            Self::WeightedDuration => Some(MeasurementDenominator::Second),
            Self::WeightedRepetitions => Some(MeasurementDenominator::Repetition),
            _ => None,
        }
    }

    pub fn operation(&self) -> MeasurementOperation {
        match self {
            Self::WeightedDuration => MeasurementOperation::Division,
            Self::WeightedRepetitions => MeasurementOperation::Multiplication,
            Self::Bodyweight | Self::Duration | Self::Repetitions => MeasurementOperation::Addition,
        }
    }
}
