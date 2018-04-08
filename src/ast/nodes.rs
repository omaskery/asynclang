
#[derive(Debug)]
pub enum TypeRef {
    Named {
        name: String,
        type_params: Vec<TypeRef>,
    },
    Tuple {
        type_refs: Vec<TypeRef>,
    },
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub type_ref: TypeRef,
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub enum Operator {
    Divide,
    Multiply,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    MemberOf {
        structure: Box<Expression>,
        member: String,
    },
    BinOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    FnCall {
        target: Box<Expression>,
        args: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Assignment {
        target: Expression,
        expr: Expression,
    },
    FnCall {
        target: Expression,
        args: Vec<Expression>,
    },
    Await(Expression),
    Loop(Vec<Statement>),
}

#[derive(Debug)]
pub enum TopLevelNode {
    GlobalDecl(VarDecl),
    FnDecl {
        name: String,
        params: Vec<VarDecl>,
        returns: TypeRef,
        body: Vec<Statement>,
        async: bool,
    },
    InterruptDecl {
        name: String,
        body: Vec<Statement>,
    },
}
