// /// This macro implements the FiniteDifferenceStencil2D trait for a struct. This struct should
// /// contain only instances of Node2D, the differential equation, boundary conditions, and any
// /// other strategy markers.
// #[proc_macro_attribute]
// pub fn stencil2d() {}
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod args;

use args::CommaSeparatedArgs;


/// Generates a struct implementing the finite difference method in 1D.
#[proc_macro]
pub fn finite_diff_1d(args: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(args as CommaSeparatedArgs);

    parsed.print_all();

    // let item = syn::parse_macro_input!(input_item as syn::Item);

    // let syn::Item::Struct(s) = item else {
    //     return syn::Error::new(
    //         item.span(),
    //         "`finite_diff_1d` can only be used on a `struct`. ",
    //     )
    //     .to_compile_error()
    //     .into();
    // };

    // let syn::Fields::Named(fields) = s.fields else {
    //     return syn::Error::new(
    //         s.fields.span(),
    //         "`finite_diff_1d` expects the `struct` it is used on to have named fields. ",
    //     )
    //     .to_compile_error()
    //     .into();
    // };

    // for field in fields.named.into_iter() {
    //     let ident = field.ident;
    //     let syn::Type::Path(path) = field.ty else {
    //         return syn::Error::new(
    //             field.ty.span(),
    //             "This type doesn't look like a type `finite_diff_1d` is expecting. ",
    //         )
    //         .to_compile_error()
    //         .into();
    //     };

    //     eprintln!("{:#?}", path);
    // }

    quote!(
        
    )
    .into()
}

/// Generates a struct implementing the finite difference method described by the struct
/// the macro is applied to.
#[proc_macro_attribute]
pub fn finite_diff_2d(args: TokenStream, input_item: TokenStream) -> TokenStream {
    eprintln!("ARGS: {}", args);

    let item = syn::parse_macro_input!(input_item as syn::Item);

    let syn::Item::Struct(s) = item else { panic!() };

    let syn::Fields::Named(fields) = s.fields else {
        panic!()
    };

    eprintln!("FIELDS: {:?}", fields);

    quote!(
        struct T;
    )
    .into()
}
