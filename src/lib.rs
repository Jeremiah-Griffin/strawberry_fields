//re export derive macro
pub use strawberry_fields_macros::StrawberryFields;
mod test;

///Implementing this trait by hand will result in erroneuous results if the struct field changes and ruins the entire point of the crate. Don't do that.
pub unsafe trait StrawberryFields {
    fn all_fields<T>(self, function: impl FnMut(T) -> bool) -> bool;
    fn all_fields_ref<T>(&self, function: impl FnMut(T) -> bool);
    fn any_fields<T>(self, function: impl FnMut(T) -> bool) -> bool;
    fn any_fields_ref<T>(&self, function: impl FnMut(T) -> bool) -> bool;
    fn fold_fields<T, Acc>(self, initial: Acc, function: impl FnMut(T, Acc) -> Acc) -> Acc;
    fn fold_fields_ref<T, Acc>(&self, initial: Acc, function: impl FnMut(T, Acc) -> Acc);
}
