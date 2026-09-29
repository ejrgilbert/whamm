//! Symbolic constraints on generic type parameters (`<T: numeric>`).
//!
//! A [`GenericConstraint`] is a point in a lattice that constrains what concrete types a
//! type parameter may be instantiated to at a match site (distinct from a *type bound*,
//! which is a concrete operand annotation like `arg0: i32`):
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
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// A constraint on a generic type parameter.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GenericConstraint {
    Exact(DataType),
    Numeric,
    Int,
    Float,
    Top,
}

impl Display for GenericConstraint {
    /// The human-readable constraint name, as written in a generic header (and used in
    /// diagnostics). The inverse of [`GenericConstraint::from_str`] for the keyword forms.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenericConstraint::Numeric => write!(f, "numeric"),
            GenericConstraint::Int => write!(f, "int"),
            GenericConstraint::Float => write!(f, "float"),
            GenericConstraint::Top => write!(f, "unknown"),
            GenericConstraint::Exact(ty) => write!(f, "{ty}"),
        }
    }
}

impl GenericConstraint {
    /// The concrete Wasm types this constraint admits.
    pub fn leaves(&self) -> Vec<DataType> {
        match self {
            GenericConstraint::Exact(ty) => vec![ty.clone()],
            GenericConstraint::Int => vec![DataType::I32, DataType::I64],
            GenericConstraint::Float => vec![DataType::F32, DataType::F64],
            GenericConstraint::Numeric | GenericConstraint::Top => {
                vec![DataType::I32, DataType::I64, DataType::F32, DataType::F64]
            }
        }
    }

    /// Whether a concrete type satisfies this constraint.
    pub fn admits(&self, ty: &DataType) -> bool {
        match self {
            GenericConstraint::Exact(exact) => exact == ty,
            GenericConstraint::Numeric => ty.is_numeric(),
            GenericConstraint::Int => is_integer(ty),
            GenericConstraint::Float => is_float(ty),
            GenericConstraint::Top => true,
        }
    }

    /// The greatest lower bound of two constraints (the constraint that satisfies both).
    /// `None` when unsatisfiable
    pub fn meet(&self, other: &GenericConstraint) -> Option<GenericConstraint> {
        use GenericConstraint::*;
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

    /// The constraint a type parameter must satisfy to be used with `op`
    pub fn from_op_requirement(op: &BinOp) -> GenericConstraint {
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
            | BinOp::LT => GenericConstraint::Numeric,
            // Bitwise/shift ops require integers.
            BinOp::LShift | BinOp::RShift | BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => {
                GenericConstraint::Int
            }
            // Equality and logical ops impose no numeric constraint in the v1 lattice.
            BinOp::EQ | BinOp::NE | BinOp::And | BinOp::Or => GenericConstraint::Top,
        }
    }
}

impl FromStr for GenericConstraint {
    type Err = String;

    fn from_str(keyword: &str) -> Result<Self, Self::Err> {
        match keyword {
            "numeric" => Ok(GenericConstraint::Numeric),
            "int" => Ok(GenericConstraint::Int),
            "float" => Ok(GenericConstraint::Float),
            other => Err(format!(
                "unknown generic constraint `{other}`; expected one of `numeric`, `int`, or `float`"
            )),
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
        assert_eq!(GenericConstraint::Numeric.leaves(), NUMERIC.to_vec());
        assert_eq!(GenericConstraint::Top.leaves(), NUMERIC.to_vec());
        assert_eq!(
            GenericConstraint::Int.leaves(),
            vec![DataType::I32, DataType::I64]
        );
        assert_eq!(
            GenericConstraint::Float.leaves(),
            vec![DataType::F32, DataType::F64]
        );
        assert_eq!(
            GenericConstraint::Exact(DataType::I64).leaves(),
            vec![DataType::I64]
        );
    }

    #[test]
    fn admits_matches_leaves() {
        for bound in [
            GenericConstraint::Numeric,
            GenericConstraint::Int,
            GenericConstraint::Float,
            GenericConstraint::Top,
            GenericConstraint::Exact(DataType::F32),
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
        use GenericConstraint::*;
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
            GenericConstraint::from_op_requirement(&BinOp::Add),
            GenericConstraint::Numeric
        );
        assert_eq!(
            GenericConstraint::from_op_requirement(&BinOp::LShift),
            GenericConstraint::Int
        );
        assert_eq!(
            GenericConstraint::from_op_requirement(&BinOp::EQ),
            GenericConstraint::Top
        );
    }
}
