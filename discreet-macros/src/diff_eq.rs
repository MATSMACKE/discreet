use discreet_common::algebra::Expression;
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
        BinOp::Mul(_) => Ok(Expression::Constant(42.0)),
        BinOp::Div(_) => Ok(Expression::Constant(42.0)),
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
    Ok(Expression::Constant(42.0))
}

fn parse_parenthesized(expr: ExprParen) -> syn::Result<Expression> {
    parse_pde_expr(*expr.expr)
}

/// This corresponds to identifiers.
fn parse_path(expr: ExprPath) -> syn::Result<Expression> {
    Ok(Expression::Constant(42.0))
}
