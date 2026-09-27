#[derive(Debug, Clone)]
pub enum QuantumGate {
    Hadamard(usize),
    CNOT(usize, usize),
    PhaseShift(usize, f64),
    Measurement(usize, String),
}

pub struct QuantumCircuitIR {
    pub qubits: usize,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuitIR {
    pub fn new(qubits: usize) -> Self {
        Self { qubits, gates: Vec::new() }
    }

    pub fn add_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }
}
