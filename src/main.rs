extern crate petgraph;

pub mod ast;
pub mod cfg;

use ast::{format::FormatAst, visitable::Visitable};

fn main() {
    use std::io::Write;

    let ast = build_test_ast();

    let mut stdout = std::io::stdout();
    let mut printer = FormatAst::new(&mut stdout);

    for node in ast.iter() {
        node.visit_mut(&mut printer);
        writeln!(std::io::stdout()).unwrap();
    }

    for node in ast.iter() {
        let result = match node {
            &ast::nodes::TopLevelNode::FnDecl { ref name, ref body, .. } => Some((name, body)),
            &ast::nodes::TopLevelNode::InterruptDecl { ref name, ref body, .. } => Some((name, body)),
            _ => None,
        };

        match result {
            Some((name, statements)) => {
                let builder = cfg::Builder::new(&name);
                let mut cfg = builder.build(statements);

                let filepath = format!("cfg_dotfiles/{}.dot", name);
                cfg.to_dotfile(filepath).unwrap();

                cfg.tidy_graph();
                let filepath = format!("cfg_dotfiles/{}.tidy.dot", name);
                cfg.to_dotfile(filepath).unwrap();
            },
            _ => {},
        }
    }
}

fn build_test_ast() -> Vec<ast::nodes::TopLevelNode> {
    use ast::nodes::*;

    vec![
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
            async: false,
            name: "delay".into(),
            params: vec![
                VarDecl {
                    name: "continuation".into(),
                    type_ref: TypeRef::Named {
                        name: "Continuation".into(),
                        type_params: vec![],
                    },
                },
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
    ]
}

