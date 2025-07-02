use std::marker::PhantomData;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Id<'id>(pub PhantomData<fn(&'id ()) -> &'id ()>);

pub struct LifetimeBrand<'id>(PhantomData<&'id Id<'id>>);

impl<'id> LifetimeBrand<'id> {
    pub fn new(_: &'id Id<'id>) -> Self {
        LifetimeBrand(PhantomData)
    }
}

impl<'id> Drop for LifetimeBrand<'id> {
    fn drop(&mut self) {}
}

#[derive(Eq, PartialEq, Debug)]
pub struct Guard<'id>(pub Id<'id>);

#[macro_export]
macro_rules! make_guard {
    () => {{
        super let branded_place = $crate::min_generativity::Id(std::marker::PhantomData);
        #[allow(unused)]
        super let lifetime_brand = $crate::min_generativity::LifetimeBrand::new(&branded_place);
        $crate::min_generativity::Guard(branded_place)
    }};
}

#[allow(unused)]
fn generative_lifetime() {
    let id1 = make_guard!();
    let id2 = make_guard!();
    // uncomment this and it fails
    // assert_eq!(id1, id2);
}
