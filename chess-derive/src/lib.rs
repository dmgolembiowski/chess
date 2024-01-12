use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ChessFactory)]
pub fn chess_factory_derive(input: TokenStream) -> TokenStream {
    let it = syn::parse(input).unwrap();
    impl_chess_factory(&it)
}

fn impl_chess_factory(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let gen = quote! {
        impl ChessFactory for #name {}
    };
    gen.into()
}

#[proc_macro_derive(StandardChess)]
pub fn standard_chess_derive(input: TokenStream) -> TokenStream {
    let it = syn::parse(input).unwrap();
    impl_standard_chess(&it)
}

fn impl_standard_chess(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let gen = quote! {
        impl StandardChess for #name {}
    };
    gen.into()
}
