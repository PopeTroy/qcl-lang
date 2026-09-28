use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameId(pub String);

#[derive(Debug, Clone)]
pub struct TensorType {
    pub rank: usize,
    pub frame: FrameId,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub enum FrameError {
    IncompatibleFrames { lhs: FrameId, rhs: FrameId, op: String },
}

pub struct TransformGraph {
    pub edges: HashMap<(FrameId, FrameId), String>,
}

impl TransformGraph {
    pub fn find_path(&self, from: &FrameId, to: &FrameId) -> Option<Vec<String>> {
        if let Some(edge) = self.edges.get(&(from.clone(), to.clone())) {
            Some(vec![edge.clone()])
        } else {
            None
        }
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
        _op: BinOp,
    ) -> Result<TensorType, FrameError> {
        // Scalars (Rank 0) are frame invariant
        if lhs.rank == 0 && rhs.rank == 0 {
            return Ok(lhs.clone());
        }

        // Vectors/Tensors must share Frame ID at point of operation
        if lhs.frame != rhs.frame {
            // Attempt Auto-Transform: Find path in Graph
            if let Some(_path) = self.transform_graph.find_path(&rhs.frame, &lhs.frame) {
                // Insert Implicit Transform Node into MIR
                let mut rhs_transformed = rhs.clone();
                rhs_transformed.frame = lhs.frame.clone();
                Ok(lhs.clone())
            } else {
                Err(FrameError::IncompatibleFrames {
                    lhs: lhs.frame.clone(),
                    rhs: rhs.frame.clone(),
                    op: "BinaryOp".to_string(),
                })
            }
        } else {
            Ok(lhs.clone())
        }
    }
}
