use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DataStruct, DeriveInputi, Ident};

//Separating these into individual derive macros is a bit messy to read and less discoverable for the user, but
//if we were to derive them all in one macro, *adding* methods would be a breaking change

///commonly used by all the derives
fn shared(input: TokenStream, macro_name: &str) -> (DataStruct, Vec<Ident>) {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Struct(data) = input.data else {
        panic!("{macro_name} may only be derived for structs");
    };

    let fields = data
        .fields
        .iter()
        .map(|f| {
            f.ident
                .clone()
                .expect("{macro_name} was derived for a tuple struct")
        })
        .collect::<Vec<Ident>>();

    (data, fields)
}

#[proc_macro_derive(AllFields)]
pub fn all_fields(input: TokenStream) -> TokenStream {
    let (data, fields) = shared(input, "AllFields");

    quote! {
        impl #input.ident{
            pub fn all_fields<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                #({
                    if function(self.#fields) == false {return false};
                })
                true
            }

            pub fn all_fields_ref<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                #({
                    if function(self.#fields) == false {return false};
                })
                true
            }
        }

    }
}

#[proc_macro_derive(AllFields)]
pub fn any_fields(input: TokenStream) -> TokenStream {
    let (data, fields) = shared(input, "AnyFields");

    quote! {
        impl #input.ident{
            pub fn any_fields<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                #({
                    if function(self.#fields) == true {return true};
                })
                false
            }

            pub fn any_fields_ref<T>(self, mut function: impl FnMut(T) -> bool) -> bool{
                #({
                    if function(self.#fields) == true {return true};
                })
                false
            }
        }

    }
}

#[proc_macro_derive(ForFields)]
pub fn for_fields(input: TokenStream) -> TokenStream {
    let (data, fields) = shared(input, "ForFields");

    quote! {
        impl #input.ident{
            ///Applies `function` to all fields of `self`
            pub fn for_fields<T>(self, mut function: impl FnMut(T)){
                #({
                    function(self.#fields);
                })

            }

            ///Applies `function` to all fields of `&self`
            pub fn for_fields_ref<T>(&self, mut function: impl FnMut(&T)){
                #({
                    function(self.#fields);
                })

            }
        }
    }
}

#[proc_macro_derive(FoldFields)]
pub fn fold_fields(input: TokenStream) -> TokenStream {
    let (data, fields) = shared(input, "FoldFields");

    quote! {
        impl #input.ident{
            pub fn fold_fields<T, Acc>(self, initial: Acc, mut function: impl FnMut(T, Acc) -> Acc) -> Acc{
                let mut accumulator = inital;
                #({
                    accumulator = function(self.#fields);
                })
                accumulator
            }

            pub fn fold_fields_ref<T, Acc>(&self, initial: Acc, mut function: impl FnMut(T, Acc) -> Acc){
                let mut accumulator = inital;
                #({
                    accumulator = function(self.#fields);
                })
                accumulator

            }
        }
    }
}
