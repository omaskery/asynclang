
use super::visitor::{Visitor, VisitorMut};
use super::nodes::*;

pub trait Visitable {
    fn visit<R, V: Visitor<R>>(&self, v: &V) -> R;
    fn visit_mut<R, V: VisitorMut<R>>(&self, v: &mut V) -> R;
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
