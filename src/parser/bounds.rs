//! Symbolic type bounds for polymorphic probe operands and type variables.
//!
//! A [`TypeBound`] is a point in a lattice that constrains what concrete types a
//! polymorphic operand (or type variable) may be instantiated to at a match site:
//!
//! ```text
//! Top                                  (unconstrained; `unknown`)
//! └─ Numeric
//!    ├─ Int   -- {i32, i64}
//!    └─ Float -- {f32, f64}
//! Exact(t)                             (a single concrete type)
//! ```
//!
//! To be extended for reference/GC types later.

use crate::parser::types::{BinOp, DataType};

/// A constraint on a polymorphic operand or type variable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeBound {
    Exact(DataType),
    Numeric,
    Int,
    Float,
    Top,
}

impl TypeBound {
    /// The concrete Wasm types this bound admits.
    pub fn leaves(&self) -> Vec<DataType> {
        match self {
            TypeBound::Exact(ty) => vec![ty.clone()],
            TypeBound::Int => vec![DataType::I32, DataType::I64],
            TypeBound::Float => vec![DataType::F32, DataType::F64],
            TypeBound::Numeric | TypeBound::Top => {
                vec![DataType::I32, DataType::I64, DataType::F32, DataType::F64]
            }
        }
    }

    /// Whether a concrete type satisfies this bound.
    pub fn admits(&self, ty: &DataType) -> bool {
        match self {
            TypeBound::Exact(exact) => exact == ty,
            TypeBound::Numeric => ty.is_numeric(),
            TypeBound::Int => is_integer(ty),
            TypeBound::Float => is_float(ty),
            TypeBound::Top => true,
        }
    }

    /// The greatest lower bound of two bounds (the constraint that satisfies both).
    /// `None` when unsatisfiable
    pub fn meet(&self, other: &TypeBound) -> Option<TypeBound> {
        use TypeBound::*;
        match (self, other) {
            (Top, x) | (x, Top) => Some(x.clone()),

            (Exact(a), Exact(b)) => (a == b).then(|| Exact(a.clone())),
            (Exact(t), class) | (class, Exact(t)) => {
                // An exact type survives a class bound only if the class admits it.
                class.admits(t).then(|| Exact(t.clone()))
            }

            // Numeric hierarchy rules
            (Numeric, Int) | (Int, Numeric) => Some(Int),
            (Numeric, Float) | (Float, Numeric) => Some(Float),

            // Incompatible categories
            (Int, Float) | (Float, Int) => None,

            // equivalent categories or unhandled lattice changes
            (a, b) => {
                if a == b {
                    Some(a.clone())
                } else {
                    todo!()
                }
            }
        }
    }

    /// Lower bound an operand type variable must satisfy to be used with `op`
    pub fn from_op_requirement(op: &BinOp) -> TypeBound {
        match op {
            // Arithmetic and ordered comparisons require a numeric operand.
            BinOp::Add
            | BinOp::Subtract
            | BinOp::Multiply
            | BinOp::Divide
            | BinOp::Modulo
            | BinOp::GE
            | BinOp::GT
            | BinOp::LE
            | BinOp::LT => TypeBound::Numeric,
            // Bitwise/shift ops require integers.
            BinOp::LShift | BinOp::RShift | BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => {
                TypeBound::Int
            }
            // Equality and logical ops impose no numeric constraint in the v1 lattice.
            BinOp::EQ | BinOp::NE | BinOp::And | BinOp::Or => TypeBound::Top,
        }
    }
}

fn is_integer(ty: &DataType) -> bool {
    matches!(ty, DataType::I32 | DataType::I64)
}

fn is_float(ty: &DataType) -> bool {
    matches!(ty, DataType::F32 | DataType::F64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMERIC: [DataType; 4] = [DataType::I32, DataType::I64, DataType::F32, DataType::F64];

    #[test]
    fn leaves_cover_the_expected_universe() {
        assert_eq!(TypeBound::Numeric.leaves(), NUMERIC.to_vec());
        assert_eq!(TypeBound::Top.leaves(), NUMERIC.to_vec());
        assert_eq!(TypeBound::Int.leaves(), vec![DataType::I32, DataType::I64]);
        assert_eq!(
            TypeBound::Float.leaves(),
            vec![DataType::F32, DataType::F64]
        );
        assert_eq!(
            TypeBound::Exact(DataType::I64).leaves(),
            vec![DataType::I64]
        );
    }

    #[test]
    fn admits_matches_leaves() {
        for bound in [
            TypeBound::Numeric,
            TypeBound::Int,
            TypeBound::Float,
            TypeBound::Top,
            TypeBound::Exact(DataType::F32),
        ] {
            for ty in NUMERIC.iter() {
                assert_eq!(
                    bound.admits(ty),
                    bound.leaves().contains(ty),
                    "{bound:?} admits {ty:?}"
                );
            }
        }
    }

    #[test]
    fn meet_is_greatest_lower_bound() {
        use TypeBound::*;
        assert_eq!(Top.meet(&Numeric), Some(Numeric));
        assert_eq!(Numeric.meet(&Int), Some(Int));
        assert_eq!(Int.meet(&Numeric), Some(Int));
        assert_eq!(Int.meet(&Float), None);
        assert_eq!(
            Numeric.meet(&Exact(DataType::I32)),
            Some(Exact(DataType::I32))
        );
        assert_eq!(Int.meet(&Exact(DataType::F32)), None);
        assert_eq!(
            Exact(DataType::I64).meet(&Exact(DataType::I64)),
            Some(Exact(DataType::I64))
        );
        assert_eq!(Exact(DataType::I32).meet(&Exact(DataType::I64)), None);
    }

    #[test]
    fn op_requirements() {
        assert_eq!(
            TypeBound::from_op_requirement(&BinOp::Add),
            TypeBound::Numeric
        );
        assert_eq!(
            TypeBound::from_op_requirement(&BinOp::LShift),
            TypeBound::Int
        );
        assert_eq!(TypeBound::from_op_requirement(&BinOp::EQ), TypeBound::Top);
    }
}
