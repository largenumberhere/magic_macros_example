extern crate proc_macro;
use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::token_stream::TokenStream;

type TokenTree1 = proc_macro::TokenTree;
type TokenTree2 = proc_macro2::TokenTree;

use macro_magic::import_tokens_attr;
use syn::spanned::Spanned;

macro_magic::export_tokens_alias!(export_struct);

// Ask macro_magic to help us out with the following code!
#[import_tokens_attr]

// Mark this as a attribute procedural macro
#[proc_macro_attribute]
pub fn combine_structs(attr1: TokenStream1, tokens: TokenStream1) -> TokenStream1{
    // Convert the `proc_macro::TokenStream`'s into Representations of structs
    let foreign_struct: ItemStruct = parse_macro_input!(attr1 as ItemStruct);
    let local_struct: ItemStruct = parse_macro_input!(tokens as ItemStruct);

    //Get all the fields of first struct or report a useful error
    let Fields::Named(local_fields) = local_struct.fields else {
        return syn::Error::new(
            local_struct.fields.span(),
            "Unnamed fields are not supported."
        ).to_compile_error().into()
    };

    //Get all the fields of the 2nd struct or report a nice error
    let Fields::Named(foreign_fields) = foreign_struct.fields else {
        return syn::Error::new(
            foreign_struct.fields.span(),
            "Unnamed fields are not supported.",
        ).to_compile_error().into()
    };

    //Prepare variables for quote! to expand bellow
    let foreign_fields = foreign_fields.named.iter();
    let local_fields = local_fields.named.iter();
    let attrs = local_struct.attrs;
    let generics = local_struct.generics;
    let ident = local_struct.ident;
    let vis = local_struct.vis;

    let result: TokenStream2 = quote!(
        //List out all the attributes that are between our attribute and before the struct's start
        #(
            #attrs
        )*

        #vis struct #ident < #generics > {
            //List out all of the fields of the first struct separated by commas
            #(
                #local_fields
            ),*

            //The first struct may not have ended with a comma
            ,

            //List out all the other fields separated by commas
            #(
                #foreign_fields
            ),*
        }
    );


    //Convert from proc_macro2::TokenStream into proc_macro:TokenStream
    result.into()
}




