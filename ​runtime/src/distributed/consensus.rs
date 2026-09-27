pub struct RaftNode {
    pub node_id: u64,
    pub current_term: u64,
}

impl RaftNode {
    pub fn new(id: u64) -> Self {
        Self { node_id: id, current_term: 0 }
    }

    pub fn request_vote(&mut self, term: u64) -> bool {
        if term > self.current_term {
            self.current_term = term;
            true
        } else {
            false
        }
    }
}
