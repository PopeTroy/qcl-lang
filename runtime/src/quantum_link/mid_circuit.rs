pub struct MidCircuitMeasurementEngine;

impl MidCircuitMeasurementEngine {
    pub fn apply_conditional_feedforward(&self, measurement_bit: bool) -> &'static str {
        if measurement_bit { "APPLY_X_GATE" } else { "NO_OP" }
    }
}
