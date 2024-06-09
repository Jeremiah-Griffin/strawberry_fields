pub use strawberry_fields_macros::strawberry_fields;
//re export derive macro
mod test;

//note: we can't do something like to_array as users may wish to use an impl trait instead of a concrete type
//for Alias, and silently boxing trait objects for them is probably a bad idea.

///Provides a `std::iter`- like API for consuming, mutating, and reading the fields of a struct.
///
///SAFETY: Implementing this trait by hand will result in erroneuous results if the struct's fields changes. This ruins the entire point of the crate. Don't do that.
pub unsafe trait StrawberryFields {
    ///The type of the struct field methods will be called on. This may be
    ///a concrete type, or, if the `impl Trait in Type Alias` feature is enabled or stablized, an `impl Trait` generic bound.
    type Argument;
    ///Returns the number of fields for this implementor. This is resolved at compile time.
    const FIELD_COUNT: usize;
    ///Applies `predicate` to each of `self`, short-circuiting and returning `false` if any element fails to satisfy it.
    ///A struct without fields returns `true`.
    fn all_fields(self, predicate: impl FnMut(Self::Argument) -> bool) -> bool;
    ///Applies `predicate` to each of `&self`, short-circuiting and returning `false` if any element fails to satisfy it.
    ///A struct without fields returns `true`.
    fn all_fields_ref(&self, predicate: impl FnMut(&Self::Argument) -> bool) -> bool;
    ///Applies `predicate` to each field of `self`, short-circuiting and returning `true` if any element satisfies it.
    ///A struct without fields returns `false`.
    fn any_fields(self, predicate: impl FnMut(Self::Argument) -> bool) -> bool;
    ///Applies `predicate` to each field of `&self`, short-circuiting and returning `true` if any element satisfies it.
    ///A struct without fields returns `false`.
    fn any_fields_ref(&self, predicate: impl FnMut(&Self::Argument) -> bool) -> bool;
    ///Attempts to find a field that satisfies the `predicate`, returning `Some(Self::Argument)` on the first one that does.
    ///This method is short-circuiting.
    ///A struct with no fields returns None.
    fn find_field(self, predicate: impl FnMut(&Self::Argument) -> bool) -> Option<Self::Argument>;
    ///Attempts to find a field that satisfies the `predicate`, returning `Some(Self::Argument)` on the first one that does.
    ///This method is short-circuiting.
    ///A struct with no fields returns None.
    ///A struct with no fields returns None.
    fn find_field_ref(
        &self,
        predicate: impl FnMut(&Self::Argument) -> bool,
    ) -> Option<&Self::Argument>;

    ///Folds every element into an accumulator by applying an operation, returning the final result.
    ///fold() takes two arguments: an initial value, and a closure with two arguments: an ‘accumulator’, and an element. The closure returns the value that the accumulator should have for the next iteration.
    ///The initial value is the value the accumulator will have on the first call.
    ///After applying this closure to every element of the iterator, fold() returns the accumulator.
    ///The method consumes `self`.
    fn fold_fields<Acc>(
        self,
        initial: Acc,
        function: impl FnMut(Self::Argument, Acc) -> Acc,
    ) -> Acc;
    ///Folds every element into an accumulator by applying an operation, returning the final result.
    ///fold() takes two arguments: an initial value, and a closure with two arguments: an ‘accumulator’, and an element. The closure returns the value that the accumulator should have for the next iteration.
    ///The initial value is the value the accumulator will have on the first call.
    ///After applying this closure to every element of the iterator, fold() returns the accumulator.
    ///The method takes `self` by reference.
    fn fold_fields_ref<Acc>(
        &self,
        initial: Acc,
        function: impl FnMut(&Self::Argument, Acc) -> Acc,
    ) -> Acc;
    ///Applies `function` to all fields of `self`, consuming it.
    fn for_fields(self, function: impl FnMut(Self::Argument));
    ///Applies `function` to all fields of `&self`
    fn for_fields_ref(&self, function: impl FnMut(&Self::Argument));
    ///Applies `function` to all fields of `&mut self`
    fn for_fields_mut(&mut self, function: impl FnMut(&mut Self::Argument));
}
