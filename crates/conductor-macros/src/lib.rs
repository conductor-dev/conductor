use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Norm)]
pub fn derive_norm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            let fields = match &data_struct.fields {
                Fields::Named(fields_named) => {
                    let fields: Vec<_> = fields_named
                        .named
                        .iter()
                        .map(|f| {
                            let field_name = &f.ident;

                            quote! {
                                #field_name: self.#field_name.norm(),
                            }
                        })
                        .collect();

                    quote! {
                        Self {
                            #(#fields)*
                        }
                    }
                }
                Fields::Unnamed(fields_unnamed) => {
                    let fields: Vec<_> = fields_unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let field_index = syn::Index::from(i);

                            quote! {
                                self.#field_index.norm(),
                            }
                        })
                        .collect();

                    quote! {
                        Self(#(#fields)*)
                    }
                }
                Fields::Unit => {
                    quote! { Self }
                }
            };

            quote! {
                impl Norm for #struct_name {
                    type Output = Self;

                    fn norm(self) -> Self::Output {
                        #fields
                    }
                }
            }
        }
        Data::Enum(_) => unimplemented!("Enum support not implemented"),
        Data::Union(_) => unimplemented!("Union support not implemented"),
    };

    TokenStream::from(expanded)
}
