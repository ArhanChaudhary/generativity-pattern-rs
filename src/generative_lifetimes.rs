use std::marker::PhantomData;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Id<'id>(PhantomData<fn(&'id ()) -> &'id ()>);

pub struct LifetimeBrand<'id>(PhantomData<&'id Id<'id>>);

impl<'id> LifetimeBrand<'id> {
    pub fn new(_: &'id Id<'id>) -> Self {
        LifetimeBrand(PhantomData)
    }
}

impl<'id> Drop for LifetimeBrand<'id> {
    fn drop(&mut self) {}
}

macro_rules! make_guard {
    () => {{
        super let branded_place = Id(PhantomData);
        #[allow(unused)]
        super let lifetime_brand = LifetimeBrand::new(&branded_place);
        branded_place
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generative_lifetime() {
        let id1 = make_guard!();
        let id2 = make_guard!();
        // uncomment this and it fails
        // assert_eq!(id1, id2);
    }
}
