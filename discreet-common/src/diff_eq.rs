pub struct Derivative<E: Expression, V: Variable> {
    _e: std::marker::PhantomData<E>,
    _v: std::marker::PhantomData<V>,
}
impl<E: Expression, V: Variable> Expression for Derivative<E, V> {}

pub struct Mul<Lhs: Expression, Rhs: Expression> {
    _l: std::marker::PhantomData<Lhs>,
    _r: std::marker::PhantomData<Rhs>,
}
impl<Lhs: Expression, V: Expression> Expression for Mul<Lhs, V> {}

pub trait Expression {}

pub trait Variable {
    fn get_name() -> String;
}
