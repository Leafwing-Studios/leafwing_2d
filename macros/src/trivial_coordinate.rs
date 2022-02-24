use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Data, DeriveInput, Ident};

pub(crate) fn trivial_coordinate_inner(ast: &DeriveInput) -> TokenStream {
    // Splitting the abstract syntax tree
    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let crate_path = if let Ok(found_crate) = crate_name("leafwing_2d") {
        // The crate was found in the Cargo.toml
        match found_crate {
            FoundCrate::Itself => quote!(leafwing_2d),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote!(#ident)
            }
        }
    } else {
        // The crate was not found in the Cargo.toml,
        // so we assume that we are in the owning_crate itself
        //
        // In order for this to play nicely with unit tests within the crate itself,
        // `use crate as leafwing_2d` at the top of each test module where this macro is needed
        //
        // Note that doc tests, integration tests and examples want the full standard import,
        // as they are evaluated as if they were external
        quote!(leafwing_2d)
    };

    // Fetch the wrapped field
    let data_struct = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("TrivialCoordinate can only be derived for struct types."),
    };

    // The first field is used as the wrapped type; all others are ignored.
    let wrapped_field = data_struct
        .fields
        .iter()
        .next()
        .expect("At least one field must be provided.");

    let wrapped_type = &wrapped_field.ty;

    // Populate the `TrivialCoordinate` trait
    // For named structs
    if let Some(field_name) = &wrapped_field.ident {
        quote! {
            impl #impl_generics #crate_path::TrivialCoordinate for #struct_name #type_generics #where_clause {
                type Wrapped = #wrapped_type;

                fn value(&self) -> Self::Wrapped {
                    self.#field_name
                }
            }
        }
    // For unnamed (tuple) structs
    } else {
        quote! {
            impl #impl_generics #crate_path::TrivialCoordinate for #struct_name #type_generics #where_clause {
                type Wrapped = #wrapped_type;

                fn value(&self) -> Self::Wrapped {
                    self.0
                }
            }
        }
    }
}
