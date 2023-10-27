use database::impl_bindable;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum Measurement {
    Bodyweight,
    Duration,
    Repetitions,
    WeightedRepetitions,
    WeightedDuration,
}

impl Default for Measurement {
    fn default() -> Self {
        Self::WeightedRepetitions
    }
}

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum MeasurementUnit {
    Kilogram,
    Repetition,
    Second,
}

impl Default for MeasurementUnit {
    fn default() -> Self {
        Self::Kilogram
    }
}

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case", type_name = "varchar")]
#[strum(serialize_all = "snake_case")]
pub enum MeasurementDenominator {
    Repetition,
    Second,
}

impl Default for MeasurementDenominator {
    fn default() -> Self {
        Self::Repetition
    }
}

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MeasurementOperation {
    Addition,
    Division,
    Multiplication,
}

impl Default for MeasurementOperation {
    fn default() -> Self {
        Self::Multiplication
    }
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

impl_bindable!(Measurement, MeasurementUnit, MeasurementDenominator);
