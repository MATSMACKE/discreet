pub struct Derivative<Expr: Expression, Variable: self::Variable> {
    _e: std::marker::PhantomData<Expr>,
    _v: std::marker::PhantomData<Variable>,
}
impl<E: Expression, V: Variable> Expression for Derivative<E, V> {
    fn eval() -> Expr {
        Expr::Derivative {
            expr: Box::new(E::eval()),
            var: V::get_name(),
        }
    }
}

enum Expr {
    Derivative { expr: Box<Self>, var: String },
}

trait Expression {
    fn eval() -> Expr;
}

pub trait Variable {
    fn get_name() -> String;
}
