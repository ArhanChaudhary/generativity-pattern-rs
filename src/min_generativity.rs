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
    #[test]
    fn generative_lifetime() {
        #![allow(unused)]
        let id1 = make_guard!();
        let id2 = make_guard!();
        // uncomment this and it fails
        // assert_eq!(id1, id2);
    }
}
