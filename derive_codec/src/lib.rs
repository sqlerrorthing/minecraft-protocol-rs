#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, parse_macro_input};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let body = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                let field_encodes = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote! {
                        self.#ident.encode(buffer).await?;
                    }
                });
                quote! {
                    #( #field_encodes )*
                }
            }
            Fields::Unnamed(ref fields) => {
                let field_encodes = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = Index::from(i);
                    quote! {
                        self.#index.encode(buffer).await?;
                    }
                });
                quote! {
                    #( #field_encodes )*
                }
            }
            Fields::Unit => quote! {},
        },

        Data::Enum(data_enum) => {
            let variant_matches = data_enum.variants.iter().enumerate().map(|(i, v)| {
                let variant = &v.ident;
                quote! {
                    #name::#variant { .. } => #i,
                }
            });

            quote! {
                let index = match self {
                    #( #variant_matches )*
                };
                VarI32(index as i32).encode(buffer).await?;
            }
        }

        _ => unimplemented!(),
    };

    let expanded = quote! {
        #[async_trait::async_trait]
        impl Encoder for #name {
            async fn encode<W>(&self, buffer: &mut W) -> Result<(), std::io::Error>
            where
                W: tokio::io::AsyncWrite + Unpin + Send,
            {
                #body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generated = match input.data {
        Data::Struct(data_struct) => {
            let decode_fields = match data_struct.fields {
                Fields::Named(ref fields) => {
                    let bindings = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! { #ident: <#ty as Decoder>::decode(buffer).await? }
                    });

                    quote! {
                        Ok(#name {
                            #( #bindings ),*
                        })
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let values = fields.unnamed.iter().map(|_| {
                        quote! { Decoder::decode(buffer).await? }
                    });
                    quote! {
                        Ok(#name(
                            #( #values ),*
                        ))
                    }
                }
                Fields::Unit => quote! {
                    Ok(#name)
                },
            };

            quote! {
                #[async_trait::async_trait]
                impl Decoder for #name {
                    async fn decode<R>(buffer: &mut R) -> Result<Self, std::io::Error>
                    where
                        R: tokio::io::AsyncRead + Unpin + Send,
                    {
                        #decode_fields
                    }
                }
            }
        }

        Data::Enum(data_enum) => {
            let variant_arms = data_enum.variants.iter().enumerate().map(|(i, v)| {
                let variant = &v.ident;

                match &v.fields {
                    Fields::Unit => {
                        quote! {
                            #i => Ok(#name::#variant),
                        }
                    }

                    _ => {
                        quote! {
                            #i => return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Only unit-like enum variants are supported in this macro",
                            )),
                        }
                    }
                }
            });

            quote! {
                impl Decoder for #name {
                    fn decode<R>(buffer: &mut R) -> Result<Self, std::io::Error>
                    where
                        R: tokio::io::AsyncRead + Unpin + Send,
                    {
                        let index = VarI32::decode(buffer).await?.0 as usize;
                        match index {
                            #( #variant_arms )*
                            _ => Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("Invalid enum variant index: {}", index),
                            )),
                        }
                    }
                }
            }
        }

        _ => unimplemented!(),
    };

    generated.into()
}
