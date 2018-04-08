
use std::io::Write;

use super::visitor::VisitorMut;
use super::nodes::*;

pub struct FormatAst<'a> {
    writer: &'a mut Write,
    indent: u32,
}

impl<'a> FormatAst<'a> {
    pub fn new(writer: &'a mut Write) -> Self {
        FormatAst {
            writer,
            indent: 0,
        }
    }

    pub fn with_indent(writer: &'a mut Write, indent: u32) -> Self {
        FormatAst {
            writer,
            indent,
        }
    }

    fn indent(&mut self) {
        for _ in 0..self.indent {
            write!(self.writer, "  ");
        }
    }

    fn indented<F: FnOnce(&mut Self)>(&mut self, f: F) {
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

impl<'a> VisitorMut<()> for FormatAst<'a> {
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

                self.indented(|s| {
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

                self.indented(|s| {
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

                self.indented(|s| {
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

