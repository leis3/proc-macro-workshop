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

    let idents = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();
    let tys = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

    let expanded = quote! {
        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#idents: None),*
                }
            }
        }

        pub struct #builder_ident {
            #(#idents: Option<#tys>),*
        }

        impl #builder_ident {
            #(
                fn #idents(&mut self, #idents: #tys) -> &mut Self {
                    self.#idents = Some(#idents);
                    self
                }
            )*
            pub fn build(&self) -> Result<#ident, Box<dyn std::error::Error>> {
                #(
                    let #idents = match &self.#idents {
                        Some(#idents) => #idents.clone(),
                        None => return Err(format!("Missing \"{}\" field", stringify!(#idents)).into())
                    };
                )*
                Ok(#ident {
                    #(#idents),*
                })
            }
        }

    };

    expanded.into()
}
