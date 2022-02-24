use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Data, DeriveInput, Ident};

pub(crate) fn trivial_coordinate_inner(ast: &DeriveInput) -> TokenStream {
    // Splitting the abstract syntax tree
    let struct_name = ast.ident.clone();
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

    assert!(
        data_struct.fields.len() == 1,
        "Exactly one field must be provided."
    );

    // The first field is used as the wrapped type; all others are ignored.
    let wrapped_field = data_struct
        .fields
        .iter()
        .next()
        .expect("Exactly one field must be provided.");

    let wrapped_type = wrapped_field.ty.clone();

    assert!(
        wrapped_field.ident.is_none(),
        "Only tuple structs can be used."
    );

    quote! {
        // Populate the `TrivialCoordinate` trait
        impl #impl_generics #crate_path::position::TrivialCoordinate for #struct_name #type_generics #where_clause {
            type Wrapped = #wrapped_type;

            fn value(&self) -> Self::Wrapped {
                self.0
            }
        }

        // Default
        impl #impl_generics std::fmt::Debug for #struct_name #type_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
                f.debug_struct(#struct_name)
                    .field("0", &self.0)
                    .finish()
            }
        }

        // Clone and Copy
        impl #impl_generics std::clone::Clone for #struct_name #type_generics #where_clause {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl #impl_generics core::marker::Copy for #struct_name #type_generics #where_clause {}

        // Default
        impl #impl_generics std::default::Default for #struct_name #type_generics #where_clause {
            fn default() -> Self {
                Self(#wrapped_type::default())
            }
        }

        // Equality and ordering
        impl #impl_generics std::cmp::PartialEq for #struct_name #type_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl #impl_generics std::cmp::PartialOrd for #struct_name #type_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        // Addition
        impl #impl_generics core::ops::Add for #struct_name #type_generics #where_clause {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl #impl_generics core::ops::AddAssign for #struct_name #type_generics #where_clause {
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0 + other.0
            }
        }

        // Subtraction
        impl #impl_generics core::ops::Sub for #struct_name #type_generics #where_clause {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl #impl_generics core::ops::SubAssign for #struct_name #type_generics #where_clause {
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0 - other.0
            }
        }

        // Multiplication
        impl #impl_generics core::ops::Mul for #struct_name #type_generics #where_clause {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl #impl_generics core::ops::MulAssign for #struct_name #type_generics #where_clause {
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0 * other.0
            }
        }

        // Division
        impl #impl_generics core::ops::Div for #struct_name #type_generics #where_clause {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl #impl_generics core::ops::DivAssign for #struct_name #type_generics #where_clause {
            fn div_assign(&mut self, other: Self) {
                self.0 = self.0 / other.0
            }
        }

        // Remainder
        impl #impl_generics core::ops::Rem for #struct_name #type_generics #where_clause {
            type Output = Self;

            fn rem(self, other: Self) -> Self {
                Self(self.0 % other.0)
            }
        }

        impl #impl_generics core::ops::RemAssign for #struct_name #type_generics #where_clause {
            fn rem_assign(&mut self, other: Self) {
                self.0 = self.0 % other.0
            }
        }
    }
}
