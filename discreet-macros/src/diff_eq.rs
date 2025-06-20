use discreet_common::algebra::Expression;
use syn::{
    Expr, ExprAssign, ExprBinary, ExprCall, ExprLit, ExprParen, ExprPath, ExprUnary, Lit, LitInt,
    spanned::Spanned,
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
        Expr::Call(expr) => parse_fn_call(expr),
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
    Ok(Expression::Constant(42.0))
}

fn parse_unary(expr: ExprUnary) -> syn::Result<Expression> {
    let operand = parse_pde_expr(*expr.expr)?;
    Ok(Expression::Constant(42.0))
}

fn parse_fn_call(expr: ExprCall) -> syn::Result<Expression> {
    Ok(Expression::Constant(42.0))
}

fn parse_literal(expr: ExprLit) -> syn::Result<Expression> {
    Ok(Expression::Constant(42.0))
}

fn parse_parenthesized(expr: ExprParen) -> syn::Result<Expression> {
    let expr = parse_pde_expr(*expr.expr)?;
    Ok(Expression::Constant(42.0))
}

fn parse_path(expr: ExprPath) -> syn::Result<Expression> {
    Ok(Expression::Constant(42.0))
}
