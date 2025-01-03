use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, ItemFn};
use quote::ToTokens;


#[proc_macro_attribute]
pub fn funclog(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.sig.ident;

    let block = ast.block.as_ref();
    let block = parse_quote! {{
        println!("Enter: {}", stringify!(#ident));

        let block = || #block;
        let ret = block();

        println!("Exit: {}", stringify!(#ident));
        ret
    }};

    *ast.block = block;
    ast.into_token_stream().into()
}