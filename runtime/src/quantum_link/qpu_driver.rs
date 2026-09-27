use anyhow::Result;

pub struct QPUDriver {
    pub target_qpu: String,
}

impl QPUDriver {
    pub fn execute_qasm(&self, qasm_code: &str) -> Result<Vec<u8>> {
        tracing::info!("Executing QASM sequence on QPU Target {}", self.target_qpu);
        Ok(qasm_code.as_bytes().to_vec())
    }
}
