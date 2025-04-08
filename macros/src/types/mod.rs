use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Colon, Comma, PathSep},
    Expr, ExprReference, Ident, ItemEnum, Path, Token,
};
mod kw {
    use syn::custom_keyword;
    custom_keyword!(module);
    custom_keyword!(function);
    custom_keyword!(matches);
}

///Parameters to a macro that look like function parameters if their names were required to be explicit.
struct NamedMacroParam<T: Parse> {
    _name: Ident,
    _colon: Token![:],
    param: T,
}

impl<T: Parse> NamedMacroParam<T> {
    pub fn new(param_name: &str, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed_name = Ident::parse(&input)?;

        if format_ident!("{param_name}") != parsed_name {
            let passed_name = parsed_name.to_string();

            panic!("Expected a parameter named: {param_name}, but found {passed_name} instead.");
        }

        Ok(NamedMacroParam {
            _name: parsed_name,
            _colon: Colon::parse(&input)?,
            param: T::parse(&input)?,
        })
    }
}

#[derive(Clone)]
pub enum ExprOrReference {
    Expression(Expr),
    Reference(ExprReference),
}

impl Parse for ExprOrReference {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.fork().parse::<ExprReference>().is_ok() {
            true => ExprReference::parse(input).map(|r| Self::Reference(r)),
            false => Expr::parse(input).map(|e| Self::Expression(e)),
        }
    }
}

impl ExprOrReference {
    fn expression_without_reference(&self) -> &Expr {
        match self {
            ExprOrReference::Expression(expr) => expr,
            ExprOrReference::Reference(expr_reference) => expr_reference.expr.as_ref(),
        }
    }
}

#[allow(dead_code)]
pub struct TestExpression {
    //Optional as references are only needed if the type signature of the function needs it.
    pub input: ExprOrReference,
    pub fat_arrow: Token![=>],
    pub pattern: Expr,
}

impl Parse for TestExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: ExprOrReference::parse(input).expect("could not parse input pattern"),
            fat_arrow: <Token![=>]>::parse(input)?,
            pattern: Expr::parse(input)?,
        })
    }
}

struct TestExpressionList {
    _brackets: Bracket,
    list: Punctuated<TestExpression, Comma>,
}

impl Parse for TestExpressionList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list_buffer: ParseBuffer<'_>;

        Ok(TestExpressionList {
            _brackets: bracketed!(list_buffer in input),
            list: Punctuated::parse_terminated(&list_buffer).expect("Parsing list failed"),
        })
    }
}
pub struct VariantsInput {
    module: NamedMacroParam<Ident>,
    _first_comma: Comma,
    function: NamedMacroParam<Path>,
    _second_comma: Comma,
    test_expressions: NamedMacroParam<TestExpressionList>,
}

impl Parse for VariantsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(VariantsInput {
            module: NamedMacroParam::new("module", input)?,
            _first_comma: Comma::parse(input)?,
            function: NamedMacroParam::new("function", input)?,
            _second_comma: Comma::parse(input)?,
            test_expressions: NamedMacroParam::new("matches", input)?,
        })
    }
}

impl VariantsInput {
    fn validate(input: TokenStream, definition: &ItemEnum) -> Self {
        let input = syn::parse::<VariantsInput>(input).expect("Failed to parse");
        let variant_count = definition.variants.len();
        let length = input.test_expressions.param.list.len();
        if length != variant_count {
            let (item_or_items, were_or_was, variant_or_variants) = match length == 1 {
                true => ("item", "was", "variant"),
                false => ("items", "were", "variants"),
            };
            panic!("{length} {item_or_items} {were_or_was} supplied but the enum has {variant_count} {variant_or_variants}")
        }
        input
    }

    ///Common functionality between the eq and neq tests
    pub fn parse_for_equality(
        input: TokenStream,
        item: TokenStream,
        assertion: Assertion,
    ) -> TokenStream {
        let enum_definition =
            syn::parse::<ItemEnum>(item).expect("test_for_variants may only be used with enums.");

        let variants_input = VariantsInput::validate(input, &enum_definition);

        let pairs_length = variants_input.test_expressions.param.list.len();
        let tested_function = variants_input.function.param;

        //the variants which will be tested

        //TODO: This is beyond horrible. Collect this into a single vector of a T that implements to tokens
        let mut variant_names: Vec<proc_macro2::TokenStream> = Vec::with_capacity(pairs_length);
        //names generated from the function name and the variant tame which will be given to each test case
        let mut test_names: Vec<Ident> = Vec::with_capacity(pairs_length);
        //The patterns to be matched
        let mut expected_values = Vec::with_capacity(pairs_length);

        for elem in variants_input.test_expressions.param.list.into_iter() {
            //generate test function names.
            {
                let variant_name_string = elem
                    .input
                    .expression_without_reference()
                    .to_token_stream()
                    .to_string()
                    //If a variant input does not have fields this will consume the entire variant input.
                    //However, if it has fields (either within parenthesis or brackets) this will short circuit consuming only
                    //the variant name.
                    .chars()
                    .take_while(|g| !g.is_whitespace() && *g != '(' && *g != '{')
                    .collect::<String>();

                let tested_function = tested_function
                    .clone()
                    .segments
                    .last()
                    .expect("The function (or path) does not have a final segment as expected.")
                    .ident
                    .clone();

                test_names.push(format_ident!("{tested_function}_{variant_name_string}"));
            }

            //format variant name from &Variant to &Fully::Qualified::Path::To::Variant
            {
                let mut variant_name = proc_macro2::TokenStream::new();

                let input_variant_expression = &elem.input;

                if let ExprOrReference::Reference(r) = input_variant_expression {
                    r.and_token.to_tokens(&mut variant_name);
                    if let Some(mutability) = r.mutability {
                        mutability.to_tokens(&mut variant_name);
                    }
                }

                enum_definition.ident.to_tokens(&mut variant_name);
                PathSep::default().to_tokens(&mut variant_name);
                input_variant_expression
                    .expression_without_reference()
                    .to_tokens(&mut variant_name);
                variant_names.push(variant_name);
            }

            expected_values.push(elem.pattern);
        }
        let module = variants_input.module.param;

        quote! {
            #enum_definition

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod #module{
                //allows paths relative to parent
                use super::*;
                #(
                    #[test]
                    fn #test_names(){
                        #assertion(#tested_function(#variant_names), #expected_values);
                    }
                )*
            }
        }
        .into()
    }
}

pub enum Assertion {
    Eq,
    Ne,
}

impl ToTokens for Assertion {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Assertion::Eq => tokens.extend(quote! {assert_eq!}),
            Assertion::Ne => tokens.extend(quote! {assert_ne!}),
        }
    }
}
