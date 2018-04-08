
use super::nodes::*;

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
