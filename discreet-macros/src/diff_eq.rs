use discreet_common::algebra::{Expression, Variable};
use syn::{
    BinOp, Expr, ExprAssign, ExprBinary, ExprCall, ExprLit, ExprParen, ExprPath, ExprUnary, Lit,
    LitInt, spanned::Spanned,
};

pub fn parse_pde(expr: Expr) -> syn::Result<Expression> {
    match expr {
        Expr::Assign(ExprAssign { left, right, .. }) => {
            let right_span = right.span();
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(litint),
                ..
            }) = *right
            {
                if litint.base10_parse::<isize>()? == 0 {
                    return parse_pde_expr(*left);
                }
            }
            Err(syn::Error::new(
                right_span,
                "Expected the RHS of the PDE to be an integer value of zero.",
            ))
        }
        _ => Err(syn::Error::new(
            expr.span(),
            "Expected the PDE to be formatted as an equation of the form `lhs = 0`.",
        )),
    }
}

fn parse_pde_expr(expr: Expr) -> syn::Result<Expression> {
    match expr {
        Expr::Binary(expr) => parse_binop(expr),
        Expr::Unary(expr) => parse_unary(expr),
        Expr::Call(expr) => Err(syn::Error::new(
            expr.span(),
            "The PDE should not contain any function calls. If you need to use a function that isn't the function you're solving for, you should simply use the function's identifier. ",
        )),
        Expr::Lit(expr) => parse_literal(expr),
        Expr::Paren(expr) => parse_parenthesized(expr),
        Expr::Path(expr) => parse_path(expr),
        _ => Err(syn::Error::new(
            expr.span(),
            "Unexpected type of expression in equation.",
        )),
    }
}

fn parse_binop(expr: ExprBinary) -> syn::Result<Expression> {
    let left = parse_pde_expr(*expr.left)?;
    let right = parse_pde_expr(*expr.right)?;

    match expr.op {
        BinOp::Add(_) => {
            let mut sum = Vec::new();
            match left {
                Expression::Sum(terms) => {
                    sum.extend(terms);
                }
                other => {
                    sum.push(other);
                }
            }
            match right {
                Expression::Sum(terms) => {
                    sum.extend(terms);
                }
                other => {
                    sum.push(other);
                }
            }
            Ok(Expression::Sum(sum))
        }
        BinOp::Sub(_) => {
            let mut sum = Vec::new();
            match left {
                Expression::Sum(terms) => {
                    sum.extend(terms);
                }
                other => {
                    sum.push(other);
                }
            }
            match right {
                Expression::Sum(mut terms) => {
                    let terms = terms
                        .into_iter()
                        .map(|exp| Expression::Negate(Box::new(exp)));
                    sum.extend(terms);
                }
                other => {
                    sum.push(Expression::Negate(Box::new(other)));
                }
            }
            Ok(Expression::Sum(sum))
        }
        BinOp::Mul(_) => {
            let mut prod = Vec::new();
            match left {
                Expression::Prod(terms) => {
                    prod.extend(terms);
                }
                other => {
                    prod.push(other);
                }
            }
            match right {
                Expression::Prod(terms) => {
                    prod.extend(terms);
                }
                other => {
                    prod.push(other);
                }
            }
            Ok(Expression::Prod(prod))
        }
        BinOp::Div(_) => {
            let mut prod = Vec::new();
            match left {
                Expression::Prod(terms) => {
                    prod.extend(terms);
                }
                other => {
                    prod.push(other);
                }
            }
            match right {
                Expression::Prod(mut terms) => {
                    let terms = terms
                        .into_iter()
                        .map(|exp| Expression::Reciprocal(Box::new(exp)));
                    prod.extend(terms);
                }
                other => {
                    prod.push(Expression::Reciprocal(Box::new(other)));
                }
            }
            Ok(Expression::Sum(prod))
        }
        // We treat `^` as exponentiation
        BinOp::BitXor(_) => Ok(Expression::Constant(42.0)),
        x => Err(syn::Error::new(x.span(), "Unexpected operator in PDE")),
    }
}

fn parse_unary(expr: ExprUnary) -> syn::Result<Expression> {
    let operand = parse_pde_expr(*expr.expr)?;
    Ok(Expression::Constant(42.0))
}

fn parse_literal(expr: ExprLit) -> syn::Result<Expression> {
    match expr.lit {
        Lit::Float(v) => Ok(Expression::Constant(v.base10_parse()?)),
        _ => Err(syn::Error::new(
            expr.span(),
            "Only floating point literals are allowed here.",
        )),
    }
}

fn parse_parenthesized(expr: ExprParen) -> syn::Result<Expression> {
    parse_pde_expr(*expr.expr)
}

/// This corresponds to identifiers.
fn parse_path(expr: ExprPath) -> syn::Result<Expression> {
    let span = expr.span();
    if expr.path.segments.len() != 1 {
        return Err(syn::Error::new(
            span,
            "Expected only identifiers without a path.",
        ));
    }

    let id = expr.path.segments.into_iter().next().unwrap().ident;

    let string = format!("{id}");

    if string.starts_with("u_") {
        let vars_string = string.split('_').nth(1).unwrap();

        let mut vars = Vec::with_capacity(vars_string.len());
        for c in vars_string.chars() {
            match Variable::from_char(c) {
                Some(v) => {
                    vars.push(v);
                }
                None => {
                    return Err(syn::Error::new(
                        span,
                        "Differentiation with respect to unknown variable. Should be x, y, z.",
                    ));
                }
            }
        }

        if vars.iter().all(|v| *v == vars[0]) {
            Ok(Expression::Derivative(vars[0], vars.len()))
        } else {
            Ok(Expression::CrossDerivative(vars))
        }
    } else if string == "u" {
        Ok(Expression::SolutionVal)
    } else {
        Ok(Expression::SymbolicConstant(id))
    }
}

#[cfg(test)]
mod test {
    use discreet_common::algebra::Expression;
    use quote::quote;
    use syn::Expr;

    use crate::diff_eq::parse_pde;

    #[test]
    fn example1() {
        let stream = quote! {du/dt - nu * d2u/dx2 - 2 * u = 0};
        let expr: Expr = syn::parse2(stream).unwrap();

        match parse_pde(expr) {
            Ok(e) => {
                assert_eq!(e, Expression::Constant(42.0))
            }
            Err(e) => {
                panic!()
            }
        }
    }
}
