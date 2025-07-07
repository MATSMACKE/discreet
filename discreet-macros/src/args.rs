use syn::{
    Expr, ExprLit, ExprPath, Ident, Lit, Path, Token, UnOp,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Minus,
};

pub struct CommaSeparatedArgs {
    items: Vec<Arg>,
}

impl CommaSeparatedArgs {
    pub fn print_all(&self) {
        for item in &self.items {
            eprintln!("ARG: {item:#?}")
        }
    }
}

impl CommaSeparatedArgs {
    pub fn find_arg(&self, ident: String) -> Option<Expr> {
        for item in &self.items {
            if item.ident == ident {
                return Some(item.value.clone());
            }
        }
        None
    }
}

impl Parse for CommaSeparatedArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Punctuated<_, Token![,]> = Punctuated::parse_terminated(input)?;
        Ok(Self {
            items: parsed.into_iter().collect(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Arg {
    ident: Ident,
    value: Expr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        let next = input.lookahead1();

        let value: Expr = if next.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            input.parse()?
        } else if next.peek(Token![,]) {
            syn::parse_str("true")?
        } else {
            return Err(next.error());
        };

        Ok(Arg { ident, value })
    }
}

pub fn parse_stencil(stencil: Expr) -> syn::Result<Vec<(isize, isize)>> {
    let span = stencil.span();
    match stencil {
        Expr::Array(a) => {
            let elems = a.elems;

            let mut stencil = Vec::with_capacity(elems.len());
            for item in elems {
                match item {
                    Expr::Tuple(e) => {
                        let mut iter = e.elems.into_iter();

                        let x = parse_int_lit(iter.next().unwrap())?;
                        let y = parse_int_lit(iter.next().unwrap())?;

                        stencil.push((x, y));
                    }
                    _ => return Err(syn::Error::new(span, "Expected tuple.")),
                }
            }

            Ok(stencil)
        }
        _ => Err(syn::Error::new(
            span,
            "Expected stencil to be an array of offsets.",
        )),
    }
}

pub fn parse_int_lit(e: Expr) -> syn::Result<isize> {
    match e {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => Ok(i.base10_parse()?),
        Expr::Unary(e) => {
            let UnOp::Neg(_) = e.op else {
                return Err(syn::Error::new(
                    e.span(),
                    format!("Expected integer, found {:?}.", e.op),
                ));
            };

            match *e.expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(i), ..
                }) => Ok(-i.base10_parse()?),
                other => Err(syn::Error::new(
                    other.span(),
                    format!("Expected integer, found {other:?}."),
                )),
            }
        }
        other => Err(syn::Error::new(
            other.span(),
            format!("Expected integer, found {other:?}."),
        )),
    }
}

pub fn ident_list(expr: Expr) -> syn::Result<Vec<Ident>> {
    let span = expr.span();
    match expr {
        Expr::Array(a) => {
            let elems = a.elems;

            let mut idents = Vec::with_capacity(elems.len());
            for item in elems {
                let id = get_ident(item)?;

                idents.push(id);
            }

            Ok(idents)
        }
        _ => Err(syn::Error::new(
            span,
            "Expected `constants` and `functions` to be an array of identifiers.",
        )),
    }
}

pub fn get_ident(expr: Expr) -> syn::Result<Ident> {
    let span = expr.span();
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            if path.segments.len() != 1 {
                return Err(syn::Error::new(
                    span,
                    "Expected `constants` and `functions` to be an array of identifiers without a path.",
                ));
            }
            return Ok(path.segments.first().unwrap().ident.clone());
        }
        _ => Err(syn::Error::new(
            span,
            "Expected `constants` and `functions` to be an array of identifiers.",
        )),
    }
}
