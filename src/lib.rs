pub use strawberry_fields_macros::strawberry_fields;
//re export derive macro
mod test;

///Implementing this trait by hand will result in erroneuous results if the struct field changes and ruins the entire point of the crate. Don't do that.
pub unsafe trait StrawberryFields {
    type Alias;
    fn all_fields(self, function: impl FnMut(Self::Alias) -> bool) -> bool;
    fn all_fields_ref(&self, function: impl FnMut(&Self::Alias) -> bool) -> bool;
    fn any_fields(self, function: impl FnMut(Self::Alias) -> bool) -> bool;
    fn any_fields_ref(&self, function: impl FnMut(&Self::Alias) -> bool) -> bool;
    fn fold_fields<Acc>(self, initial: Acc, function: impl FnMut(Self::Alias, Acc) -> Acc) -> Acc;
    fn fold_fields_ref<Acc>(
        &self,
        initial: Acc,
        function: impl FnMut(&Self::Alias, Acc) -> Acc,
    ) -> Acc;
    fn for_fields(self, function: impl FnMut(Self::Alias));
    fn for_fields_ref(&self, function: impl FnMut(&Self::Alias));
}

mod blah {
    use strawberry_fields_macros::strawberry_fields;

    use crate::{self as strawberry_fields};

    #[strawberry_fields(u32)]
    pub struct Dummy {
        first: u32,
        second: u32,
    }
}
