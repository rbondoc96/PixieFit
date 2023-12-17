use crate::enums::{Measurement, MeasurementDenominator, MeasurementOperation};
use serde::Serialize;

#[derive(Serialize)]
pub struct MeasurementResource {
    unit: &'static str,
    denominator: Option<MeasurementDenominator>,
    operation: MeasurementOperation,
}

impl MeasurementResource {
    pub fn new(measurement: Measurement) -> Self {
        Self {
            unit: measurement.unit(),
            denominator: measurement.denominator(),
            operation: measurement.operation(),
        }
    }
}
