use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Expr, Ident, Token};


pub struct CommaSeparatedArgs {
    items: Vec<Arg>
}

impl CommaSeparatedArgs {
    pub fn print_all(&self) {
        for item in &self.items {
            eprintln!("ARG: {:?}", item)
        }
    }
}


impl Parse for CommaSeparatedArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Punctuated<_, Token![,]> = Punctuated::parse_terminated(&input)?;
        Ok(Self {items: parsed.into_iter().collect()})
    }
}

#[derive(Debug)]
pub struct Arg {
    ident: Ident,
    value: Expr
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        let _: Token![:] = input.parse()?;

        let value: Expr = input.parse()?;

        Ok(Arg {ident, value})
    }
}
