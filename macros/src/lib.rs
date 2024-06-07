use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(StrawberryFields)]
pub fn strawberry_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let Data::Struct(data) = input.data else {
        panic!("StrawberryFields may only be derived for structs");
    };

    let fields = data
        .fields
        .iter()
        .map(|f| {
            f.ident
                .clone()
                .expect("StrawberryFields was derived for a tuple struct")
        })
        .collect::<Vec<Ident>>();

    quote! {

         unsafe impl strawberry_fields::StrawberryFields for #name {
                    pub fn all_fields<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                        #(
                            if function(self.#fields) == false {return false};
                        )*
                        true
                    }

                    pub fn all_fields_ref<T>(&self, mut function: impl FnMut(T) -> bool) -> bool{
                        #(
                            if function(self.#fields) == false {return false};
                        )*
                        true
                    }

                pub fn all_fields<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                    #(
                        if function(self.#fields) == false {return false};
                    )*
                    true
                }

                pub fn all_fields_ref<T>(&self, mut function: impl FnMut(T) -> bool) -> bool{
                    #(
                        if function(self.#fields) == false {return false};
                    )*
                    true
                }
                pub fn any_fields<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                    #(
                        if function(self.#fields) == true {return true};
                    )*
                    false
                }

                pub fn any_fields_ref<T>(&self, mut function: impl FnMut(T) -> bool) -> bool{
                    #(
                        if function(self.#fields) == true {return true};
                    )*
                    false
                }

                            ///Applies `function` to all fields of `self`
                pub fn for_fields<T>(self, mut function: impl FnMut(T)){
                    #(
                        function(self.#fields);
                    )*

                }

                ///Applies `function` to all fields of `&self`
                pub fn for_fields_ref<T>(&self, mut function: impl FnMut(&T)){
                    #(
                        function(self.#fields);
                    )*
                }

                pub fn fold_fields<T, Acc>(self, initial: Acc, mut function: impl FnMut(T, Acc) -> Acc) -> Acc{
                    let mut accumulator = inital;
                    #(
                        accumulator = function(self.#fields);
                    )*
                    accumulator
                }

                pub fn fold_fields_ref<T, Acc>(&self, initial: Acc, mut function: impl FnMut(T, Acc) -> Acc){
                    let mut accumulator = inital;
                    #(
                        accumulator = function(self.#fields);
                    )*
                    accumulator

                }
            }

        }
    .into()
}
