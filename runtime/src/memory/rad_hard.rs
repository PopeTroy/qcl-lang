pub struct TripleModularRedundancy<T: Copy + PartialEq> {
    node_a: T,
    node_b: T,
    node_c: T,
}

impl<T: Copy + PartialEq> TripleModularRedundancy<T> {
    pub fn new(val: T) -> Self {
        Self { node_a: val, node_b: val, node_c: val }
    }

    pub fn read_voted(&self) -> T {
        if self.node_a == self.node_b || self.node_a == self.node_c {
            self.node_a
        } else {
            self.node_b
        }
    }
}
