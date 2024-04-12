use crate::{Literal, Type};

#[derive(Debug, Clone)]
pub enum Expr {
    // Atomic
    Literal(Literal),
    Type(Type),
    Ident(String),

    // Arithmetic

    // Binary
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Mod(Box<Self>, Box<Self>),

    // Unary
    Neg(Box<Self>),
    Pos(Box<Self>),

    IncR(Box<Self>),
    IncL(Box<Self>),
    DecR(Box<Self>),
    DecL(Box<Self>),

    Factorial(Box<Self>),

    Deref(Box<Self>),
    Address(Box<Self>),

    // Comparison
    Eq(Box<Self>, Box<Self>),
    Ne(Box<Self>, Box<Self>),
    Lt(Box<Self>, Box<Self>),
    Le(Box<Self>, Box<Self>),
    Gt(Box<Self>, Box<Self>),
    Ge(Box<Self>, Box<Self>),

    // Logical
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Not(Box<Self>),

    // Bitwise
    BitAnd(Box<Self>, Box<Self>),
    BitOr(Box<Self>, Box<Self>),
    BitXor(Box<Self>, Box<Self>),
    BitNot(Box<Self>),
    Shl(Box<Self>, Box<Self>),
    Shr(Box<Self>, Box<Self>),

    // Member
    Member(Box<Self>, Box<Self>),

    // Cast
    Cast(Box<Self>, Box<Self>),
    Size(Box<Self>),
    Align(Box<Self>),

    // Assignments
    Assign(Box<Self>, Box<Self>),
    AddAssign(Box<Self>, Box<Self>),
    SubAssign(Box<Self>, Box<Self>),
    MulAssign(Box<Self>, Box<Self>),
    DivAssign(Box<Self>, Box<Self>),
    ModAssign(Box<Self>, Box<Self>),
    ShlAssign(Box<Self>, Box<Self>),
    ShrAssign(Box<Self>, Box<Self>),
    BitAndAssign(Box<Self>, Box<Self>),
    BitXorAssign(Box<Self>, Box<Self>),
    BitOrAssign(Box<Self>, Box<Self>),

    // Call
    Call(Box<Self>, Vec<Self>),

    ArrMember(Box<Self>),

    // Sequence
    Tuple(Vec<Self>),
    Array(Vec<Self>),

    Unknown, // For variable initialization
}
