use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse, Ident, ItemEnum, ItemStruct, Type};
use types::VariantsInput;

mod types;

#[proc_macro_attribute]
///Generates an implementation for `StrawberryFields` in a safe way that is guaranteed to 
///be total over all struct fields.
pub fn strawberry_fields(type_parameter: TokenStream, input: TokenStream) -> TokenStream {
    if type_parameter.is_empty() {
        panic!("Type parameter must not be empty.")
    }
    let type_parameter: Type = parse(type_parameter)
        .expect("Found a non type or generic parameter in type position.");

    let data: ItemStruct = parse(input).expect("StrawberryFields may only be derived for structs.");

    let struct_definition = data.clone();
    let name = data.ident;
    let (impl_generics, type_generics, where_clause)= data.generics.split_for_impl();

    let fields = data
        .fields
        .iter()
        .enumerate()
        .map(|(_i, f)| {
            match f.ident.clone() {
                //some case is a struct
                Some(ident) => ident,
                //tuple structs have anonymous fields
                //None => Ident::new(i.to_string().as_str(), Span::call_site().into()),
                None => unreachable!("Macro should have checked that it isn't expanding for a tuple stuct, but did not.")
            }
        })
        .collect::<Vec<Ident>>();

    let field_count = fields.len();

    quote! {
    #struct_definition

     unsafe impl #impl_generics strawberry_fields::StrawberryFields for #name #type_generics #where_clause{
            type Argument = #type_parameter;

            const FIELD_COUNT: usize = #field_count;

            fn all_fields(self, mut predicate: impl FnMut(Self::Argument) -> bool) -> bool{
                #(
                    if !predicate(self.#fields) {return false};
                )*
                true
            }

            fn all_fields_ref(&self, mut predicate: impl FnMut(&Self::Argument) -> bool) -> bool{
                #(
                    if !predicate(&self.#fields){return false};
                )*
                true
            }

            fn any_fields(self, mut predicate: impl FnMut(Self::Argument) -> bool) -> bool{
                #(
                    if predicate(self.#fields) {return true};
                )*
                false
            }

            fn any_fields_ref(&self, mut predicate: impl FnMut(&Self::Argument) -> bool) -> bool{
                #(
                    if predicate(&self.#fields) {return true};
                )*
                false
            }

            fn find_field(
                self,
                mut predicate: impl FnMut(&Self::Argument) -> bool,
            ) -> Option<Self::Argument>{
                #(
                    if predicate(&self.#fields) {return Some(self.#fields)};
                )*
                None
            }

            fn find_field_ref(
                &self,
                mut predicate: impl FnMut(&Self::Argument) -> bool,
            ) -> Option<&Self::Argument>{
                 #(
                    if predicate(&self.#fields) {return Some(&self.#fields)};
                )*
                None
            }

            fn fold_fields<Acc>(self, initial: Acc, mut predicate: impl FnMut(Self::Argument, Acc) -> Acc) -> Acc{
                let mut accumulator = initial;
                #(
                    accumulator = predicate(self.#fields, accumulator);
                )*
                accumulator
            }

            fn fold_fields_ref<Acc>(&self, initial: Acc, mut predicate: impl FnMut(&Self::Argument, Acc) -> Acc) -> Acc{
                let mut accumulator = initial;
                #(
                    accumulator = predicate(&self.#fields, accumulator);
                )*
                accumulator
            }

            fn for_fields(self, mut function: impl FnMut(Self::Argument)){
                #(
                    function(self.#fields);
                )*

            }

            fn for_fields_ref(&self, mut function: impl FnMut(&Self::Argument)){
                #(
                    function(&self.#fields);
                )*
            }
            
            fn for_fields_mut(&mut self, mut function: impl FnMut(&mut Self::Argument)){
                #(
                    function(&mut self.#fields);
                )*
            }
        }
    }
    .into()
}


///Maybe the API shoukd be the num variant, a function name which consumes the variant, and a pattern , that if matched, returns successfully.
///Also have a variant that should fail when matching that pattern.
///So far as importing the tested function is concerned, the function must be available in the parent scope of 
///the module this macro generates. This is because we import the function from super.
///
///TODO: For associated functions (on the enum itself) we should be able to skip this generation if the root of the path is either `Self` or the enum name.
///TODO: How do we make methods testable? Maybe just not concern ourselves with that and instead methods can be a wrapper of a function. Idk. 

#[proc_macro_attribute]
///TODO: take a name for each function testing an enum variant
///TODO: take 
pub fn test_variants_eq(pairs: TokenStream, item: TokenStream) -> TokenStream {

    let enum_definition = parse::<ItemEnum>(item).expect("test_for_variants may only be used with enums.");

    
    let pairs = VariantsInput::validate(pairs, enum_definition.clone().variants.len());

    let pairs_length = pairs.list.len();
    let tested_function = pairs.function_name;
    
    let mut variant_names = Vec::with_capacity(pairs_length);
    let mut test_names = Vec::with_capacity(pairs_length);
    let mut expected_values = Vec::with_capacity(pairs_length);

    for elem in pairs.list.into_iter(){
        let variant_name_string = elem.input.clone().to_token_stream().to_string();
        let tested_function_clone = tested_function.clone();

        
        test_names.push(format_ident!("{tested_function_clone}_{variant_name_string}"));
        variant_names.push(elem.input);
        expected_values.push(elem.pattern);
    }
    let enum_name = enum_definition.ident.clone();
    //We add some randomness to the test module name so that users can generate mulitple sets of tests for the same enum.
    let module = pairs.module_name;





    quote!{
        #enum_definition

        //allowed because either rustc thinks that when we pass in an enum variant to the macro it's a function. No clue really why.
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod #module{
            use super::*;
            #(
                #[test]
                fn #test_names(){
                    assert_eq!(#tested_function(#enum_name::#variant_names), #expected_values);
                }
            )*
        }    
    }.into()    
}
