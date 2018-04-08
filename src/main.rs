
mod ast;

use ast::{nodes::*, format::FormatAst, visitable::Visitable};

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
    let mut printer = FormatAst::new(&mut stdout);
    for node in ast {
        node.visit_mut(&mut printer);
    }
}
