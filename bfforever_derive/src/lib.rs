use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Meta, Path, parse::Parser, parse_macro_input, punctuated::Punctuated, Token, Type};
use quote::quote;

#[proc_macro_derive(ZObjectReader)]
pub fn derive_zobjectreader(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let fields = get_struct_fields(&input);
    

    for field in fields {
        let att = field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
        let typ;

        match &field.ty {
            Type::Path(path) => {
                typ = path.path.segments.last().as_ref().map(|s| s.ident.to_string()).unwrap_or_default();
            },
            _ => unimplemented!()
        }

        println!("{att} ({typ})");
    }

    //"fn answer() -> u32 { 42 }".parse().unwrap()

    quote!().into()
}

fn get_struct_fields<'a>(input: &'a DeriveInput) -> Vec<&'a Field> {
    let Data::Struct(data_struct) = &input.data else {
        panic!("Only structs are supported")
    };

    let Fields::Named(fields) = &data_struct.fields else {
        panic!("Only named fields are supported")
    };

    fields
        .named
        .iter()
        .collect()
}