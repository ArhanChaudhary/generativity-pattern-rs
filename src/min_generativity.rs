use std::marker::PhantomData;

pub type Id<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

#[derive(Eq, PartialEq, Debug)]
pub struct Guard<'id>(pub Id<'id>);

impl<'id> From<Guard<'id>> for Id<'id> {
    fn from(guard: Guard<'id>) -> Self {
        guard.0
    }
}

pub struct LifetimeBrand<'id>(PhantomData<&'id Id<'id>>);

impl<'id> LifetimeBrand<'id> {
    pub fn new(_: &'id Id<'id>) -> Self {
        LifetimeBrand(PhantomData)
    }
}

impl<'id> Drop for LifetimeBrand<'id> {
    fn drop(&mut self) {}
}

#[macro_export]
macro_rules! make_guard {
    // ($name:ident) => {
    //     let branded_place: $crate::min_generativity::Id = std::marker::PhantomData;
    //     let lifetime_brand = $crate::min_generativity::LifetimeBrand::new(&branded_place);
    //     let $name = $crate::min_generativity::Guard(branded_place);
    // };
    () => {{
        super let branded_place: $crate::min_generativity::Id = std::marker::PhantomData;
        super let lifetime_brand = $crate::min_generativity::LifetimeBrand::new(&branded_place);
        $crate::min_generativity::Guard(branded_place)
    }};
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use std::panic::{RefUnwindSafe, UnwindSafe};

    #[test]
    fn generative_lifetime() {
        let id1 = make_guard!();
        let id2 = make_guard!();
        assert_eq!(id1, id1);
        assert_eq!(id2, id2);
        // uncomment this and it doesn't compile
        // assert_eq!(id1, id2);
    }

    #[test]
    fn test_oibits() {
        fn assert_oibits<T>(_: &T)
        where
            T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe,
        {
        }

        let a = make_guard!();
        assert_oibits(&a);
        let id: Id<'_> = a.into();
        assert_oibits(&id);

        // const compatible (e.g. const_refs_to_cell, const destructor)
        const fn _const_id(_: Id<'_>) {}
        const fn _const_ref_id(_: &'_ Id<'_>) {}
        const fn _const_guard(_: Guard<'_>) {}
        const fn _const_ref_guard(_: &'_ Guard<'_>) {}
    }
}
