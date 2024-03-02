mod strand_enum;
mod strand_struct;

use strand_enum::strand_derive_enum;
use strand_struct::strand_derive_struct;

use proc_macro::TokenStream;

#[proc_macro_derive(Strand, attributes(strand))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(_) => strand_derive_struct(input),
        syn::Data::Enum(_) => strand_derive_enum(input),
        syn::Data::Union(_) => panic!("Cannot be a union"),
    }
}