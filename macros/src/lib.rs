use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Ident, ItemStruct, Type};

#[proc_macro_attribute]
pub fn strawberry_fields(type_parameter: TokenStream, input: TokenStream) -> TokenStream {
    if type_parameter.is_empty() {
        panic!("Type parameter of StrawberryFields must not be empty.")
    }
    let type_parameter: Type = parse(type_parameter)
        .expect("Found a non type parameter in the type position of StrawberryFields.");

    let data: ItemStruct = parse(input).expect("StrawberryFields may only be derived for structs.");

    let struct_definition = data.clone();
    let name = data.ident;

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

    quote! {

    #struct_definition


     unsafe impl strawberry_fields::StrawberryFields for #name {
            type Alias = #type_parameter;

            fn all_fields(self, mut function: impl FnMut(Self::Alias) -> bool) -> bool{
                #(
                    if !function(self.#fields) {return false};
                )*
                true
            }

            fn all_fields_ref(&self, mut function: impl FnMut(&Self::Alias) -> bool) -> bool{
                #(
                    if !function(&self.#fields){return false};
                )*
                true
            }

            fn any_fields(self, mut function: impl FnMut(Self::Alias) -> bool) -> bool{
                #(
                    if function(self.#fields) {return true};
                )*
                false
            }

            fn any_fields_ref(&self, mut function: impl FnMut(&Self::Alias) -> bool) -> bool{
                #(
                    if function(&self.#fields) {return true};
                )*
                false
            }

            ///Applies `function` to all fields of `self`
            fn for_fields(self, mut function: impl FnMut(Self::Alias)){
                #(
                    function(self.#fields);
                )*

            }

            ///Applies `function` to all fields of `&self`
            fn for_fields_ref(&self, mut function: impl FnMut(&Self::Alias)){
                #(
                    function(&self.#fields);
                )*
            }

            fn fold_fields<Acc>(self, initial: Acc, mut function: impl FnMut(Self::Alias, Acc) -> Acc) -> Acc{
                let mut accumulator = initial;
                #(
                    accumulator = function(self.#fields, accumulator);
                )*
                accumulator
            }

            fn fold_fields_ref<Acc>(&self, initial: Acc, mut function: impl FnMut(&Self::Alias, Acc) -> Acc) -> Acc{
                let mut accumulator = initial;
                #(
                    accumulator = function(&self.#fields, accumulator);
                )*
                accumulator
            }
        }

    }
    .into()
}
