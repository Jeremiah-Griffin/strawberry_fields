use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::{self, Parse}, token::Comma, Ident, ItemEnum, ItemStruct, Signature, Type};
use types::{Assertion, NamedMacroParam, TestExpressionList, TestVariantsInput};

mod types;

#[proc_macro_attribute]
///Generates an implementation for `StrawberryFields` in a safe way that is guaranteed to 
///be total over all struct fields.
pub fn strawberry_fields(type_parameter: TokenStream, input: TokenStream) -> TokenStream {
    if type_parameter.is_empty() {
        panic!("Type parameter must not be empty.")
    }
    let type_parameter: Type = syn::parse(type_parameter)
        .expect("Found a non type or generic parameter in type position.");

    let data: ItemStruct = syn::parse(input).expect("StrawberryFields may only be derived for structs.");

    

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


#[proc_macro_attribute]
pub fn list_variants(variant_type: TokenStream, input: TokenStream) -> TokenStream{

    let variant_type: Type = syn::parse(variant_type)
        .expect("Found a non type or generic parameter in type position.");
    
    let item: ItemEnum = syn::parse(input).expect("list_variants must be used on an enum definition.");

    let enum_name = item.ident;
    let (impl_generics, type_generics, where_clause)= item.generics.split_for_impl();

    let variants = item.variants.clone().into_iter().map(|v| v.ident);
    let discriminants = item.variants.into_iter().map(|v| v.discriminant.expect("List Variants can only be used when all discriminants are explicitly defined").1).collect::<Vec<_>>();

    let variant_count = discriminants.len();
    let indices = 0..variant_count;

    let test_name = format_ident!("{enum_name}_variants");

    quote!{
        impl #impl_generics #enum_name for #type_generics #where_clause{
            const VARIANTS: [#variant_type; #variant_count] = [#(#discriminants),*];
        }


        #[cfg(test)]
        #[test]
        fn #test_name(){

            #(
                assert_eq!(Self::#variants as #variant_type,  VARIANTS[#indices]);
            )*
        }
    }.into()
}


///Maybe the API shoukd be the num variant, a function name which consumes the variant, and a pattern , that if matched, returns successfully.
///Also have a variant that should fail when matching that pattern.
///So far as importing the tested function is concerned, the function must be available in the parent scope of 
///the module this macro generates. This is because we import the function from super.
///
///TODO: For associated functions (on the enum itself) we should be able to skip this generation if the root of the path is either `Self` or the enum name.
///TODO: How do we make methods testable? Maybe just not concern ourselves with that and instead methods can be a wrapper of a function. Idk. 

#[proc_macro_attribute]
///Tests that *all* variants equal the corresponding pattern.
pub fn test_variants_eq(input: TokenStream, item: TokenStream) -> TokenStream {
    TestVariantsInput::parse_for_equality(input, item, Assertion::Eq)
}


#[proc_macro_attribute]
///Tests that *all* variants do not equal the corresponding pattern.
pub fn test_variants_ne(input: TokenStream, item: TokenStream) -> TokenStream {
    TestVariantsInput::parse_for_equality(input, item, Assertion::Ne)
}

#[proc_macro_attribute]
pub fn for_each_variant(input: TokenStream, item: TokenStream) -> TokenStream{

    struct ForEachVariantInput{
        pub signature: NamedMacroParam<Signature>,
        _first_comma: Comma,
        pub list: NamedMacroParam<TestExpressionList>,        
    }

    impl Parse for ForEachVariantInput{
        fn parse(input: parse::ParseStream) -> syn::Result<Self> {
            Ok(ForEachVariantInput { signature: NamedMacroParam::new("function name:", input)?, _first_comma: Comma::parse(input)?, list: NamedMacroParam::new("", input)? })
        }        
    }

    impl ForEachVariantInput{
        fn validate(input: TokenStream, definition: &ItemEnum) -> Self {
            let input = syn::parse::<ForEachVariantInput>(input).expect("Failed to parse");
            let variant_count = definition.variants.len();
            let length = input.list.param.list.len();
            if length != variant_count {
                let (item_or_items, were_or_was, variant_or_variants) = match length == 1 {
                    true => ("item", "was", "variant"),
                    false => ("items", "were", "variants"),
                };
                panic!("{length} {item_or_items} {were_or_was} supplied but the enum has {variant_count} {variant_or_variants}")
            } input
        }
    }

    let definition = syn::parse::<ItemEnum>(item).unwrap();
    
    //TODO: ensure variants input are unique.
    let input  = ForEachVariantInput::validate(input, &definition);
    let signature = input.signature.param;
    let variants = definition.variants.into_pairs().into_iter().map(|pair| pair.into_value()).collect::<Vec<_>>();
    let (mut left, mut right) = (Vec::with_capacity(variants.len()), Vec::with_capacity(variants.len()));
    input.list.param.list.into_pairs().map(|p| p.into_value()).for_each(|e|{
        left.push(e.input);
        right.push(e.pattern);
    });

    quote! {
            #signature {
                #(#left {#right}),*
            }
    }.into()    
    
}
