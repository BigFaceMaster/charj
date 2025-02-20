use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Integer { value: BigInt },
    Float { value: f64 },
    Boolean { value: bool },
    String { value: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MIRKind {
    Call {},
    Return,
    JMP,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExprKind {
    Var { value: String },
    Call,
    Print { value: String },
}

pub enum TerminatorKind {}
