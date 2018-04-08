use petgraph::{stable_graph::StableGraph};

use std::fmt;

use super::super::ast;

pub struct Block<'a> {
    pub debug_name: String,
    pub statements: Vec<&'a ast::nodes::Statement>,
}

impl<'a> Block<'a> {
    pub fn with_name(debug_name: String) -> Self {
        Block {
            debug_name,
            statements: Vec::new(),
        }
    }
}

impl<'a> fmt::Debug for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ast::visitable::Visitable;

        write!(f, "{}", self.debug_name)?;
        if self.statements.len() > 0 {
            writeln!(f, ":")?;
        }

        let mut first = true;
        for statement in self.statements.iter() {
            if first == false {
                writeln!(f)?;
            }
            first = false;

            let mut buffer = Vec::new();
            {
                let mut format = ast::format::FormatAst::with_indent(&mut buffer, 1);
                statement.visit_mut(&mut format);
            }
            let buffer = String::from_utf8_lossy(&buffer);
            write!(f, "{}", buffer)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Edge {
    Jump,
    Await,
    IfTrue,
    IfFalse,
    Break,
    Continue,
}

pub type GraphType<'a> = StableGraph<Block<'a>, Edge>;

