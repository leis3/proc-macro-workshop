use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let builder_ident = format_ident!("{}Builder", ident);

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return syn::Error::new_spanned(ident, "Must be struct type")
                .to_compile_error()
                .into();
        }
    };

    let mut builder_fields = Vec::new();
    let mut builder_init = Vec::new();
    
    let fields = match &data.fields {
        syn::Fields::Named(named) => &named.named,
        syn::Fields::Unnamed(_) => {
            return syn::Error::new_spanned(ident, "Unnamed fields of a tuple struct or tuple variant are not supported")
                .to_compile_error()
                .into();
        },
        syn::Fields::Unit => {
            return syn::Error::new_spanned(ident, "Unit struct or unit variant are not supported")
                .to_compile_error()
                .into();
        }
    };

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        builder_fields.push(quote! {
            #ident: Option<#ty>
        });
        builder_init.push(quote! {
            #ident: None
        });
    }

    let expanded = quote! {
        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_init),*
                }
            }
        }

        pub struct #builder_ident {
            #(#builder_fields),*
        }
    };

    expanded.into()
}
