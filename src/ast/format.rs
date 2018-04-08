
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
            write!(self.writer, "  ").unwrap();
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
                write!(self.writer, ", ").unwrap();
            }
            first = false;

            f(self, &item);
        }
    }

    fn code_block(&mut self, prefix: &str, statements: &[Statement]) {
        write!(self.writer, "{}{{", prefix).unwrap();

        if statements.is_empty() == false {
            writeln!(self.writer).unwrap();
        }

        self.indented(|s| {
            for statement in statements {
                s.accept_statement(&statement);
                writeln!(s.writer).unwrap();
            }
        });

        if statements.is_empty() == false {
            self.indent();
        }

        write!(self.writer, "}}").unwrap();
    }
}

impl<'a> VisitorMut<()> for FormatAst<'a> {
    fn accept_type_ref(&mut self, x: &TypeRef) -> () {
        match x {
            &TypeRef::Named { ref name, ref type_params } => {
                write!(self.writer, "{}", name).unwrap();
                if type_params.is_empty() == false {
                    write!(self.writer, "<").unwrap();
                    self.comma_separated(
                        type_params,
                        |s, i| s.accept_type_ref(i),
                    );
                    write!(self.writer, ">").unwrap();
                }
            }
            &TypeRef::Tuple { ref type_refs } => {
                write!(self.writer, "(").unwrap();
                self.comma_separated(
                    type_refs,
                    |s, i| s.accept_type_ref(i),
                );
                write!(self.writer, ")").unwrap();
            }
        }
    }

    fn accept_var_decl(&mut self, x: &VarDecl) -> () {
        write!(self.writer, "{}: ", x.name).unwrap();
        self.accept_type_ref(&x.type_ref);
    }

    fn accept_literal(&mut self, x: &Literal) -> () {
        match x {
            &Literal::Boolean(ref b) => write!(self.writer, "{}", b).unwrap(),
            &Literal::Integer(ref i) => write!(self.writer, "{}", i).unwrap(),
            &Literal::Float(ref f) => write!(self.writer, "{}", f).unwrap(),
            &Literal::String(ref s) => write!(self.writer, "{}", s).unwrap(),
        };
    }

    fn accept_operator(&mut self, x: &Operator) -> () {
        match x {
            &Operator::Divide => write!(self.writer, "/").unwrap(),
            &Operator::Multiply => write!(self.writer, "*").unwrap(),
            &Operator::Add => write!(self.writer, "+").unwrap(),
            &Operator::Subtract => write!(self.writer, "-").unwrap(),
            &Operator::ShiftLeft => write!(self.writer, "<<").unwrap(),
            &Operator::ShiftRight => write!(self.writer, ">>").unwrap(),
            &Operator::LessThan => write!(self.writer, "<").unwrap(),
            &Operator::LessThanEqual => write!(self.writer, "<=").unwrap(),
            &Operator::GreaterThan => write!(self.writer, ">").unwrap(),
            &Operator::GreaterThanEqual => write!(self.writer, ">=").unwrap(),
            &Operator::Equal => write!(self.writer, "==").unwrap(),
            &Operator::NotEqual => write!(self.writer, "!=").unwrap(),
            &Operator::BitwiseAnd => write!(self.writer, "&").unwrap(),
            &Operator::BitwiseXor => write!(self.writer, "^").unwrap(),
            &Operator::BitwiseOr => write!(self.writer, "|").unwrap(),
            &Operator::LogicalAnd => write!(self.writer, "&&").unwrap(),
            &Operator::LogicalOr => write!(self.writer, "||").unwrap(),
        };
    }

    fn accept_expression(&mut self, x: &Expression) -> () {
        match x {
            &Expression::Identifier(ref identifier) => {
                write!(self.writer, "{}", identifier).unwrap();
            }
            &Expression::BinOp { ref left, ref operator, ref right } => {
                self.accept_expression(&*left);
                write!(self.writer, " ").unwrap();
                self.accept_operator(operator);
                write!(self.writer, " ").unwrap();
                self.accept_expression(&*right);
            }
            &Expression::FnCall { ref target, ref args } => {
                self.accept_expression(&*target);
                write!(self.writer, "(").unwrap();
                self.comma_separated(
                    args,
                    |s, i| s.accept_expression(i),
                );
                write!(self.writer, ")").unwrap();
            }
            &Expression::Literal(ref literal) => self.accept_literal(literal),
            &Expression::MemberOf { ref structure, ref member } => {
                self.accept_expression(&*structure);
                write!(self.writer, ".{}", member).unwrap();
            }
        };
    }

    fn accept_statement(&mut self, x: &Statement) -> () {
        self.indent();
        match x {
            &Statement::FnCall { ref target, ref args } => {
                self.accept_expression(&*target);
                write!(self.writer, "(").unwrap();
                self.comma_separated(
                    args,
                    |s, i| s.accept_expression(i),
                );
                write!(self.writer, ");").unwrap();
            }
            &Statement::Assignment { ref target, ref expr } => {
                self.accept_expression(&*target);
                write!(self.writer, " = ").unwrap();
                self.accept_expression(&*expr);
                write!(self.writer, ";").unwrap();
            }
            &Statement::Await(ref expr) => {
                write!(self.writer, "await ").unwrap();
                self.accept_expression(expr);
                write!(self.writer, ";").unwrap();
            }
            &Statement::Loop(ref statements) => {
                self.code_block("loop ", statements);
            }
        }
    }

    fn accept_top_level_node(&mut self, x: &TopLevelNode) -> () {
        match x {
            &TopLevelNode::GlobalDecl(ref vardecl) => {
                write!(self.writer, "global ").unwrap();
                self.accept_var_decl(vardecl);
            }
            &TopLevelNode::FnDecl { ref name, ref params, ref returns, ref body, async } => {
                if async {
                    write!(self.writer, "async ").unwrap();
                } else {
                    write!(self.writer, "fn ").unwrap();
                }
                write!(self.writer, "{}(", name).unwrap();
                self.comma_separated(
                    params,
                    |s, i| s.accept_var_decl(i),
                );
                write!(self.writer, ") ").unwrap();

                let is_none = match returns {
                    &TypeRef::Tuple { ref type_refs } => type_refs.is_empty(),
                    _ => false,
                };

                if is_none == false {
                    write!(self.writer, " ->").unwrap();
                    self.accept_type_ref(returns);
                }

                self.code_block("", body);
            }
            &TopLevelNode::InterruptDecl { ref name, ref body } => {
                self.code_block(&format!("interrupt {} ", name), body);
            }
        }
    }
}

