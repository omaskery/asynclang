
pub trait Visitor<R> {
    fn accept_type_ref(&self, x: &TypeRef) -> R;
    fn accept_var_decl(&self, x: &VarDecl) -> R;
    fn accept_literal(&self, x: &Literal) -> R;
    fn accept_operator(&self, x: &Operator) -> R;
    fn accept_expression(&self, x: &Expression) -> R;
    fn accept_statement(&self, x: &Statement) -> R;
    fn accept_top_level_node(&self, x: &TopLevelNode) -> R;
}

pub trait VisitorMut<R> {
    fn accept_type_ref(&mut self, x: &TypeRef) -> R;
    fn accept_var_decl(&mut self, x: &VarDecl) -> R;
    fn accept_literal(&mut self, x: &Literal) -> R;
    fn accept_operator(&mut self, x: &Operator) -> R;
    fn accept_expression(&mut self, x: &Expression) -> R;
    fn accept_statement(&mut self, x: &Statement) -> R;
    fn accept_top_level_node(&mut self, x: &TopLevelNode) -> R;
}

pub trait Visitable {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R;
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R;
}

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

impl Visitable for TypeRef {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_type_ref(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_type_ref(self)
    }
}

impl Visitable for VarDecl {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_var_decl(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_var_decl(self)
    }
}

impl Visitable for Literal {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_literal(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_literal(self)
    }
}

impl Visitable for Operator {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_operator(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_operator(self)
    }
}

impl Visitable for Expression {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_expression(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_expression(self)
    }
}

impl Visitable for Statement {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_statement(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_statement(self)
    }
}

impl Visitable for TopLevelNode {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R {
        v.accept_top_level_node(self)
    }
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R {
        v.accept_top_level_node(self)
    }
}
