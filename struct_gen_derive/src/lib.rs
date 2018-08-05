extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Ty::Path;
use syn::{Body, Ident, Variant, VariantData};

#[proc_macro_derive(StructIterator)]
pub fn struct_iterator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
        Body::Enum(ref variants) => panic!("Enum unsporrted."),
        Body::Struct(ref variants) => impl_struct_iter(name, variants),
    };
    gen.parse().unwrap()
}

fn impl_struct_iter(name: &Ident, variants: &VariantData) -> quote::Tokens {
    let mut result = Vec::new();

    match variants {
        VariantData::Tuple(ref fields) => {
            for (idx, variant) in fields.iter().enumerate() {
                match &variant.ty {
                    Path(ref inside, ref mo) => {
                        let ident = &mo.segments[0].ident;
                        result.push(ident);
                    }
                    _ => panic!("Unexpected tuple field."),
                }
            }
        }

        _ => panic!("Unsupported variant data."),
    }

    let mut res = Vec::new();

    for i in 0..10 {
        for x in result.iter() {
            let size = i as usize;
            res.push(quote! {
                impl_zero!(
                    [#x; #size], [<#x>::default(); #size]
                );
            });
        }
    }

    quote! {
        #(#res)*
    }
}
