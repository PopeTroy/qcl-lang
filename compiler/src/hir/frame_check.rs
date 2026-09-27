use std::collections::HashMap;
use thiserror::Error;

pub type FrameId = String;

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("Incompatible Frames: LHS frame '{lhs}' does not match RHS frame '{rhs}'")]
    IncompatibleFrames { lhs: FrameId, rhs: FrameId },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorType {
    pub rank: usize,
    pub frame: FrameId,
    pub dimension_id: String,
}

pub struct TransformGraph {
    pub edges: HashMap<FrameId, FrameId>,
}

impl TransformGraph {
    pub fn new() -> Self {
        Self { edges: HashMap::new() }
    }

    pub fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if from == to {
            return Some(vec![from.to_string()]);
        }
        let mut current = from;
        let mut path = vec![current.to_string()];

        while let Some(next) = self.edges.get(current) {
            path.push(next.clone());
            if next == to {
                return Some(path);
            }
            current = next;
        }
        None
    }
}

pub struct FrameChecker {
    pub transform_graph: TransformGraph,
}

impl FrameChecker {
    pub fn check_binary_op(
        &self,
        lhs: &TensorType,
        rhs: &TensorType,
    ) -> Result<TensorType, FrameError> {
        if lhs.rank == 0 && rhs.rank == 0 {
            return Ok(lhs.clone());
        }

        if lhs.frame != rhs.frame {
            if let Some(_path) = self.transform_graph.find_path(&rhs.frame, &lhs.frame) {
                // Auto-transform path exists; coerce to LHS frame
                Ok(lhs.clone())
            } else {
                Err(FrameError::IncompatibleFrames {
                    lhs: lhs.frame.clone(),
                    rhs: rhs.frame.clone(),
                })
            }
        } else {
            Ok(lhs.clone())
        }
    }
}
