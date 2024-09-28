use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse, punctuated::Punctuated, ExprTuple, Ident, ItemEnum, ItemStruct, Token, Type};

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

struct VariantsInput{
    ///Element 0 is a the bare variant name (no qualified path)
    ///Element 1 is the expression we're going to be matching against. 
    list: Punctuated<ExprTuple, Token![,]>,
}

impl VariantsInput{

    fn validate(input: TokenStream) -> Self{
        let pairs: VariantsInput = parse(input).expect("Expected list of tuples.");
            

        for e in pairs.list.iter(){

            let element_string = e.to_token_stream().to_string();

            let tuple_pair_count = e.elems.len();

            if tuple_pair_count != 2 {
                panic!("All elements should have a length of 2 ( a key and a value) but {element_string} is {tuple_pair_count}")
            }            
            
        
        };
        pairs    
    }

}

impl Parse for VariantsInput{
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Punctuated::parse_terminated(input).map(|list| Self{list})        
    }
}

#[proc_macro_attribute]
///TODO: take a name for each function testing an enum variant
///TODO: take 
pub fn test_variants_eq(pairs: TokenStream, item: TokenStream) -> TokenStream {
    let pairs = VariantsInput::validate(pairs);

    let mut variant_names = Vec::with_capacity(pairs.list.len());
    let mut test_expressions = Vec::with_capacity(pairs.list.len());

    for elem in pairs.list.into_iter().map(|e| e.elems){
        let length = elem.len();
        if length != 2{
            panic!("expected a tuple of the variant name and the expression to be tested. Found a different number of {length} elements instead.")
        }
        //length is checked above to be two, so these unwraps should never panic.
        variant_names.push(elem.first().unwrap().clone());
        test_expressions.push(elem.last().unwrap().clone());
    }

    let enum_definition = parse::<ItemEnum>(item).expect("test_for_variants may only be used with enums.");

    let enum_name = enum_definition.ident.clone();

    //We add some randomness to the test module name so that users can generate mulitple sets of tests for the same enum.
    let module_name = Ident::new(format!("strawberry_fields_generated_variants_test_insert_name_here").as_str(), Span::call_site().into());

    quote!{

        #enum_definition

        
        #[cfg(test)]
        mod #module_name{
            #(
                fn #variant_names(){
                    assert_eq!(super::#enum_name::#variant_names, #test_expressions);                
                }
            )*
        }    
    }.into()    
}
