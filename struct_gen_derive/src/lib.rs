extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Ty::Path;
use syn::{Body, VariantData};

/// struct_iterator
///
/// # struct_iterator
/// Comments and boilerplate from [dtolnay's presentation at Mozilla][dtolnay],
/// basic struct iteration based off of [Christopher Breeden's blog post][blog].
///
/// [dtolnay]: https://air.mozilla.org/rust-meetup-december-2016-12-15/
/// [blog]: https://cbreeden.github.io/Macros11/
#[proc_macro_derive(StructIterator)]
pub fn struct_iterator(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = match ast.body {
        Body::Enum(_) => panic!("Enum unsporrted."),
        Body::Struct(ref fields) => impl_struct_iter(fields),
    };

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_struct_iter(fields: &VariantData) -> quote::Tokens {
    // capture all the types to impl Zero on
    let mut idents = Vec::new();

    match fields {
        VariantData::Tuple(ref fields) => {
            for (_, field) in fields.iter().enumerate() {
                match &field.ty {
                    Path(_, ref f) => {
                        let ident = &f.segments[0].ident;
                        idents.push(ident);
                    }
                    _ => panic!("Unexpected tuple field."),
                }
            }
        }

        _ => panic!("Unsupported variant data."),
    }

    // the resolved code
    let mut res = Vec::new();

    // If greater than 10, use a std::vec::Vec.
    for i in 0..10 {
        for x in idents.iter() {
            let size = i as usize;
            res.push(quote! {
                impl_zero!(
                    [#x; #size], [<#x>::default(); #size]
                );
            });
        }
    }

    //
    quote! {
        #(#res)*
    }
}
