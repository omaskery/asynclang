use std::io::Write;

mod ast {
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
}

use ast::*;

struct FormatAst<'a> {
    writer: &'a mut Write,
    indent: u32,
}

impl<'a> FormatAst<'a> {
    fn indent(&mut self) {
        for _ in 0..self.indent {
            write!(self.writer, "  ");
        }
    }

    fn with_indent<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.indent += 1;
        f(self);
        self.indent -= 1;
    }

    fn comma_separated<E, I: IntoIterator<Item=E>, F: FnMut(&mut Self, &E)>(&mut self, i: I, mut f: F) {
        let mut first = true;
        for item in i.into_iter() {
            if first == false {
                write!(self.writer, ", ");
            }
            first = false;

            f(self, &item);
        }
    }
}

impl<'a> ast::VisitorMut<()> for FormatAst<'a> {
    fn accept_type_ref(&mut self, x: &TypeRef) -> () {
        match x {
            &TypeRef::Named { ref name, ref type_params } => {
                write!(self.writer, "{}", name);
                if type_params.is_empty() == false {
                    write!(self.writer, "<");
                    self.comma_separated(
                        type_params,
                        |s, i| s.accept_type_ref(i),
                    );
                    write!(self.writer, ">");
                }
            }
            &TypeRef::Tuple { ref type_refs } => {
                write!(self.writer, "(");
                self.comma_separated(
                    type_refs,
                    |s, i| s.accept_type_ref(i),
                );
                write!(self.writer, ")");
            }
        }
    }

    fn accept_var_decl(&mut self, x: &VarDecl) -> () {
        write!(self.writer, "{}: ", x.name);
        self.accept_type_ref(&x.type_ref);
    }

    fn accept_literal(&mut self, x: &Literal) -> () {
        match x {
            &Literal::Boolean(ref b) => write!(self.writer, "{}", b),
            &Literal::Integer(ref i) => write!(self.writer, "{}", i),
            &Literal::Float(ref f) => write!(self.writer, "{}", f),
            &Literal::String(ref s) => write!(self.writer, "{}", s),
        };
    }

    fn accept_operator(&mut self, x: &Operator) -> () {
        match x {
            &Operator::Divide => write!(self.writer, "/"),
            &Operator::Multiply => write!(self.writer, "*"),
            &Operator::Add => write!(self.writer, "+"),
            &Operator::Subtract => write!(self.writer, "-"),
            &Operator::ShiftLeft => write!(self.writer, "<<"),
            &Operator::ShiftRight => write!(self.writer, ">>"),
            &Operator::LessThan => write!(self.writer, "<"),
            &Operator::LessThanEqual => write!(self.writer, "<="),
            &Operator::GreaterThan => write!(self.writer, ">"),
            &Operator::GreaterThanEqual => write!(self.writer, ">="),
            &Operator::Equal => write!(self.writer, "=="),
            &Operator::NotEqual => write!(self.writer, "!="),
            &Operator::BitwiseAnd => write!(self.writer, "&"),
            &Operator::BitwiseXor => write!(self.writer, "^"),
            &Operator::BitwiseOr => write!(self.writer, "|"),
            &Operator::LogicalAnd => write!(self.writer, "&&"),
            &Operator::LogicalOr => write!(self.writer, "||"),
        };
    }

    fn accept_expression(&mut self, x: &Expression) -> () {
        match x {
            &Expression::Identifier(ref identifier) => {
                write!(self.writer, "{}", identifier);
            }
            &Expression::BinOp { ref left, ref operator, ref right } => {
                self.accept_expression(&*left);
                write!(self.writer, " ");
                self.accept_operator(operator);
                write!(self.writer, " ");
                self.accept_expression(&*right);
            }
            &Expression::FnCall { ref target, ref args } => {
                self.accept_expression(&*target);
                write!(self.writer, "(");
                self.comma_separated(
                    args,
                    |s, i| s.accept_expression(i),
                );
                write!(self.writer, ")");
            }
            &Expression::Literal(ref literal) => self.accept_literal(literal),
            &Expression::MemberOf { ref structure, ref member } => {
                self.accept_expression(&*structure);
                write!(self.writer, ".{}", member);
            }
        };
    }

    fn accept_statement(&mut self, x: &Statement) -> () {
        self.indent();
        match x {
            &Statement::FnCall { ref target, ref args } => {
                self.accept_expression(&*target);
                write!(self.writer, "(");
                self.comma_separated(
                    args,
                    |s, i| s.accept_expression(i),
                );
                writeln!(self.writer, ");");
            }
            &Statement::Assignment { ref target, ref expr } => {
                self.accept_expression(&*target);
                write!(self.writer, " = ");
                self.accept_expression(&*expr);
                writeln!(self.writer, ";");
            }
            &Statement::Await(ref expr) => {
                write!(self.writer, "await ");
                self.accept_expression(expr);
                writeln!(self.writer, ";");
            }
            &Statement::Loop(ref statements) => {
                write!(self.writer, "loop {{");

                if statements.is_empty() == false {
                    writeln!(self.writer);
                }

                self.with_indent(|s| {
                    for statement in statements {
                        s.accept_statement(&statement);
                    }
                });

                if statements.is_empty() == false {
                    self.indent();
                }

                writeln!(self.writer, "}}");
            }
        }
    }

    fn accept_top_level_node(&mut self, x: &TopLevelNode) -> () {
        match x {
            &TopLevelNode::GlobalDecl(ref vardecl) => {
                write!(self.writer, "global ");
                self.accept_var_decl(vardecl);
                writeln!(self.writer);
            }
            &TopLevelNode::FnDecl { ref name, ref params, ref returns, ref body, async } => {
                if async {
                    write!(self.writer, "async ");
                } else {
                    write!(self.writer, "fn ");
                }
                write!(self.writer, "{}(", name);
                self.comma_separated(
                    params,
                    |s, i| s.accept_var_decl(i),
                );
                write!(self.writer, ") ");

                let is_none = match returns {
                    &TypeRef::Tuple { ref type_refs } => type_refs.is_empty(),
                    _ => false,
                };

                if is_none {
                    write!(self.writer, "{{");
                } else {
                    self.accept_type_ref(&returns);
                    write!(self.writer, " {{");
                }

                if body.is_empty() == false {
                    writeln!(self.writer);
                }

                self.with_indent(|s| {
                    for statement in body {
                        s.accept_statement(&statement);
                    }
                });

                writeln!(self.writer, "}}");
            }
            &TopLevelNode::InterruptDecl { ref name, ref body } => {
                write!(self.writer, "interrupt {} {{", name);

                if body.is_empty() == false {
                    writeln!(self.writer);
                }

                self.with_indent(|s| {
                    for statement in body {
                        s.accept_statement(&statement);
                    }
                });

                writeln!(self.writer, "}}");
            }
        }
        writeln!(self.writer);
    }
}

fn main() {
    let ast = vec![
        TopLevelNode::GlobalDecl(VarDecl {
            name: "timerx_continuation".into(),
            type_ref: TypeRef::Named {
                name: "Continuation".into(),
                type_params: vec![],
            },
        }),
        TopLevelNode::FnDecl {
            async: false,
            name: "init".into(),
            params: vec![],
            returns: TypeRef::Tuple {
                type_refs: vec![],
            },
            body: vec![],
        },
        TopLevelNode::FnDecl {
            async: false,
            name: "idle".into(),
            params: vec![],
            returns: TypeRef::Tuple {
                type_refs: vec![],
            },
            body: vec![],
        },
        TopLevelNode::FnDecl {
            async: true,
            name: "delay".into(),
            params: vec![
                VarDecl {
                    name: "period_ms".into(),
                    type_ref: TypeRef::Named {
                        name: "u32".into(),
                        type_params: vec![],
                    },
                },
            ],
            returns: TypeRef::Tuple {
                type_refs: vec![],
            },
            body: vec![
                Statement::Assignment {
                    target: Expression::Identifier("timerx_continuation".into()),
                    expr: Expression::MemberOf {
                        structure: Box::new(Expression::FnCall {
                            target: Box::new(Expression::Identifier("task_current".into())),
                            args: vec![],
                        }),
                        member: "continuesWith".into(),
                    },
                },
                Statement::FnCall {
                    target: Expression::Identifier("init_timerX".into()),
                    args: vec![
                        Expression::Identifier("period_ms".into()),
                    ],
                },
            ],
        },
        TopLevelNode::FnDecl {
            async: true,
            name: "periodic".into(),
            params: vec![
                VarDecl {
                    name: "period_ms".into(),
                    type_ref: TypeRef::Named {
                        name: "u32".into(),
                        type_params: vec![],
                    },
                },
            ],
            returns: TypeRef::Tuple {
                type_refs: vec![],
            },
            body: vec![
                Statement::Loop(vec![
                    Statement::Await(
                        Expression::FnCall {
                            target: Box::new(Expression::Identifier("delay".into())),
                            args: vec![
                                Expression::Identifier("period_ms".into()),
                            ],
                        }
                    ),
                    Statement::Await(
                        Expression::FnCall {
                            target: Box::new(Expression::Identifier("println".into())),
                            args: vec![
                                Expression::Literal(Literal::String("Hi!".into())),
                            ],
                        }
                    ),
                ]),
            ],
        },
        TopLevelNode::InterruptDecl {
            name: "timerx_overflow".into(),
            body: vec![
                Statement::Await(
                    Expression::Identifier("timerx_continuation".into())
                ),
            ],
        },
    ];

    let mut stdout = std::io::stdout();
    let mut printer = FormatAst {
        writer: &mut stdout,
        indent: 0,
    };
    for node in ast {
        node.visit_mut(&mut printer);
    }
}
