extern crate proc_macro;

use proc_macro::TokenStream;

use syn::parse::{Parse, ParseBuffer, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_macro_input, token, Expr, Token};

use quote::quote;

/// Parses (Expr, Expr)
struct Pair {
    _paren: token::Paren,
    first: Expr,
    _comma: Token![,],
    second: Expr,
}

impl Parse for Pair {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self> {
        let content;
        Ok(Pair {
            _paren: parenthesized!(content in input),
            first: content.parse()?,
            _comma: content.parse()?,
            second: content.parse()?,
        })
    }
}

/// Parses (Expr, Expr),*
struct Pairs {
    pairs: Punctuated<Pair, Token![,]>,
}

impl Parse for Pairs {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self> {
        Ok(Pairs {
            pairs: input.parse_terminated(Pair::parse)?,
        })
    }
}

#[proc_macro]
pub fn btreemap_proc(input: TokenStream) -> TokenStream {
    let Pairs { pairs } = parse_macro_input!(input as Pairs);

    let mut keys: Vec<Expr> = Vec::with_capacity(pairs.len());
    let mut values: Vec<Expr> = Vec::with_capacity(pairs.len());

    for pair in pairs {
        keys.push(pair.first);
        values.push(pair.second)
    }

    let res = quote! { {
        let mut map = std::collections::BTreeMap::new();

        #(
            map.insert(#keys, #values);
        )*

        map
    }};

    res.into()
}
