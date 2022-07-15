use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, DataStruct, DeriveInput, Ident};

#[proc_macro_derive(DdsType, attributes(key))]
pub fn derive_dds_type(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let is_key = has_key_attribute(&input.attrs);

    let struct_with_key_attributes = match &input.data {
        syn::Data::Struct(struct_data) => struct_data
            .fields
            .iter()
            .find(|field| has_key_attribute(&field.attrs))
            .is_some(),
        syn::Data::Enum(enum_data) => {
            if enum_data
                .variants
                .iter()
                .find(|variant| has_key_attribute(&variant.attrs))
                .is_some()
            {
                return quote_spanned!(input.span() => compile_error!("An enum variant cannot be a key")).into();
            }
            false
        }
        syn::Data::Union(_) => {
            return quote_spanned!(input.span() => compile_error!("DdsType doesn't support derive for unions")).into();
        }
    };

    if is_key && struct_with_key_attributes {
        return quote_spanned!(input.span() => compile_error!("Using #[key] on fields is undefined when the whole struct is already marked with #[key]")).into();
    }

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let ident = input.ident;
    let type_name = ident.to_string();

    if is_key {
        quote! {
            impl #impl_generics DdsType for #ident #type_generics #where_clause {
                fn type_name() -> &'static str {
                    #type_name
                }

                fn has_key() -> bool {
                    true
                }

                fn get_serialized_key<E: dds_implementation::dds_type::Endianness>(&self) -> Vec<u8> {
                    if E::REPRESENTATION_IDENTIFIER == dds_implementation::dds_type::PL_CDR_BE {
                        cdr::serialize::<_, _, cdr::CdrBe>(self, cdr::Infinite).unwrap()
                    } else {
                        cdr::serialize::<_, _, cdr::CdrLe>(self, cdr::Infinite).unwrap()
                    }
                }

                fn set_key_fields_from_serialized_key(&mut self, key: &[u8]) -> dds_api::return_type::DdsResult<()> {
                    let mut buf = key.to_owned();
                    *self = cdr::deserialize(&mut buf).map_err(|e| dds_api::return_type::DdsError::PreconditionNotMet(e.to_string()))?;
                    Ok(())
                }
            }
        }
        .into()
    } else if struct_with_key_attributes {
        let struct_data = if let syn::Data::Struct(struct_data) = input.data {
            struct_data
        } else {
            unreachable!()
        };
        let build_key = struct_build_key(&struct_data);
        let set_key = struct_set_key(&struct_data);

        quote! {
            impl #impl_generics DdsType for #ident #type_generics #where_clause {
                fn type_name() -> &'static str {
                    #type_name
                }

                fn has_key() -> bool {
                    true
                }

                fn get_serialized_key<E: dds_implementation::dds_type::Endianness>(&self) -> Vec<u8> {
                    #build_key
                }

                fn set_key_fields_from_serialized_key(&mut self, key: &[u8]) -> dds_api::return_type::DdsResult<()> {
                    #set_key
                    Ok(())
                }
            }
        }
        .into()
    } else {
        quote! {
            impl #impl_generics DdsType for #ident #type_generics #where_clause {
                fn type_name() -> &'static str {
                    #type_name
                }

                fn has_key() -> bool {
                    false
                }

                fn get_serialized_key<E: dds_implementation::dds_type::Endianness>(&self) -> Vec<u8> {
                    vec![]
                }

                fn set_key_fields_from_serialized_key(&mut self, _key: &[u8]) -> dds_api::return_type::DdsResult<()> {
                    Ok(())
                }
            }
        }
        .into()
    }
}

fn struct_build_key(struct_data: &DataStruct) -> TokenStream2 {
    let indexed_key_fields = struct_data
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| has_key_attribute(&field.attrs));

    let mut field_list_ts = quote! {};
    for (i, field) in indexed_key_fields {
        let field_ident = field
            .ident
            .clone()
            .map(|field| field.into_token_stream())
            .unwrap_or(syn::Index::from(i).into_token_stream());

        field_list_ts.extend(quote! {self.#field_ident,});
    }

    quote! {
        if E::REPRESENTATION_IDENTIFIER == dds_implementation::dds_type::PL_CDR_BE {
            cdr::serialize::<_, _, cdr::CdrBe>(&(#field_list_ts), cdr::Infinite).unwrap()
        } else {
            cdr::serialize::<_, _, cdr::CdrLe>(&(#field_list_ts), cdr::Infinite).unwrap()
        }
    }
    .into()
}

fn struct_set_key(struct_data: &DataStruct) -> TokenStream2 {
    let indexed_key_fields = struct_data
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| has_key_attribute(&field.attrs))
        .collect::<Vec<_>>();

    let identifiers = (0..indexed_key_fields.len())
        .map(|i| Ident::new(&format!("__field_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let mut identifier_list_ts = quote! {};
    for ident in identifiers.iter() {
        identifier_list_ts.extend(quote! {#ident,});
    }

    let mut token_stream = quote! {
        let mut __buf = key.to_owned();
        let (#identifier_list_ts) = cdr::deserialize(&mut __buf).map_err(|e| dds_api::return_type::DdsError::PreconditionNotMet(e.to_string()))?;
    };

    for (&(i, field), ident) in indexed_key_fields.iter().zip(identifiers.iter()) {
        let field_ident = field
            .ident
            .clone()
            .map(|field| field.into_token_stream())
            .unwrap_or(syn::Index::from(i).into_token_stream());

        token_stream.extend(quote! {
            self.#field_ident = #ident;
        });
    }

    token_stream
}

fn has_key_attribute<'a>(attr_list: &'a [Attribute]) -> bool {
    attr_list
        .iter()
        .find(|attr| {
            attr.parse_meta()
                .ok()
                .and_then(|meta| meta.path().get_ident().cloned())
                .map(|ident| ident.to_string() == "key")
                .unwrap_or(false)
        })
        .is_some()
}
