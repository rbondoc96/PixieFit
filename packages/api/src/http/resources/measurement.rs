use crate::enums::Measurement;
use serde::Serialize;

#[derive(Serialize)]
pub struct MeasurementResource {
    unit: &'static str,
    denominator: Option<&'static str>,
    operation: &'static str,
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
