use generativity::{Guard, Id, make_guard};

/// The Branded Vector Example from §2 of the paper.
/// Run from the `./ghostcell-examples` directory, with the command
/// `cargo run --example branded_vec`.

struct BrandedVec<'id, T> {
    inner: Vec<T>,
    id: Id<'id>,
}

#[derive(Clone, Copy)]
struct BrandedIndex<'id> {
    idx: usize,
    id: Id<'id>,
}

impl<'id, T> BrandedVec<'id, T> {
    pub fn new(inner: Vec<T>, guard: Guard<'id>) -> BrandedVec<'id, T> {
        BrandedVec {
            inner,
            id: guard.into(),
        }
    }

    pub fn get_index(&self, index: usize) -> Option<BrandedIndex<'id>> {
        if index < self.inner.len() {
            Some(BrandedIndex {
                idx: index,
                id: self.id,
            })
        } else {
            None
        }
    }

    pub fn get(&self, index: BrandedIndex<'id>) -> &T {
        unsafe { self.inner.get_unchecked(index.idx) }
    }

    pub fn get_mut<'a>(&'a mut self, index: BrandedIndex<'id>) -> &'a mut T {
        unsafe { self.inner.get_unchecked_mut(index.idx) }
    }

    pub fn push<'a>(&'a mut self, val: T) -> BrandedIndex<'id> {
        let index = BrandedIndex {
            idx: self.inner.len(),
            id: self.id,
        };
        self.inner.push(val);
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branded_vec_example() {
        let mut vec: Vec<u8> = vec![0, 1, 2];
        vec.push(3);
        println!("{vec:?}"); // Prints [0, 1, 2, 3]

        // an immutable reference into the vector
        let v0: &u8 = &vec[0];
        // cannot borrow `vec` as mutable because it is also borrowed as immutable
        // vec.push(4);
        println!("{v0:?}");

        // a mutable reference into the vector
        let v1: &mut u8 = &mut vec[1];
        // cannot borrow `vec` as mutable more than once at a time
        // vec.push(4);
        *v1 += 1;
        println!("{vec:?}"); // Prints [0, 2, 2, 3]

        let vec1: Vec<u8> = vec![10, 11];
        let vec2: Vec<u8> = vec![20, 21];

        make_guard!(guard1);
        let mut bvec1 = BrandedVec::new(vec1, guard1);
        bvec1.push(12);
        let i1 = bvec1.push(13);
        let _idx = bvec1.get_index(0).unwrap();

        make_guard!(guard2);
        let mut bvec2 = BrandedVec::new(vec2, guard2);
        let i2 = bvec2.push(22);
        println!("{:?}", bvec2.get(i2)); // No bound check! Prints 22
        *bvec2.get_mut(i2) -= 1; // No bound check!
        println!("{:?}", bvec2.get(i2)); // Prints 21
        println!("{:?}", bvec1.get(i1)); // Prints 13
        // rejected: i1 is not an index of bvec2
        // println!("{:?}", bvec2.get(i1));
    }
}
