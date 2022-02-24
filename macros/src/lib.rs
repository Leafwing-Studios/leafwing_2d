//! Derives the [`Actionlike`] trait
//
//! This derive macro was inspired by the `strum` crate's `EnumIter` macro.
//! Original source: https://github.com/Peternator7/strum,
//! Copyright (c) 2019 Peter Glotfelty under the MIT License

extern crate proc_macro;
mod trivial_coordinate;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(TrivialCoordinate)]
pub fn trivialcoordinate(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    crate::trivial_coordinate::trivial_coordinate_inner(&ast).into()
}
