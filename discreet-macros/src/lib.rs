// /// This macro implements the FiniteDifferenceStencil2D trait for a struct. This struct should
// /// contain only instances of Node2D, the differential equation, boundary conditions, and any
// /// other strategy markers.
// #[proc_macro_attribute]
// pub fn stencil2d() {}
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn variable(item: TokenStream) -> TokenStream {
    let ident = syn::parse_macro_input!(item as syn::Ident);
    let name = ident.to_string();

    quote! {
        struct #ident;

        impl Variable for #ident {
            fn get_name() -> String {
                String::from(#name)
            }
        }
    }
    .into()
}
