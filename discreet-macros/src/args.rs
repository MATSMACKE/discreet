use syn::{
    Expr, Ident, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct CommaSeparatedArgs {
    items: Vec<Arg>,
}

impl CommaSeparatedArgs {
    pub fn print_all(&self) {
        for item in &self.items {
            eprintln!("ARG: {:#?}", item)
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
