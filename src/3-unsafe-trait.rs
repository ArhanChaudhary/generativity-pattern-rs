pub struct PermGroup {
    base_permutation_length: usize,
    base_permutations: Vec<Permutation>,
}

impl PermGroup {
    pub fn new(
        base_permutation_length: usize,
        base_permutation_mappings: Vec<Vec<usize>>,
    ) -> Result<Self, &'static str> {
        // ... validate that each mapping is a valid
        // permutation of the given length
        // (remember that permutations can only be
        // composed if they have the same length)
        Ok(Self {
            base_permutation_length,
            base_permutations: base_permutation_mappings
                .into_iter()
                .map(Permutation::from_mapping)
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn base_permutations(&self) -> &[Permutation] {
        &self.base_permutations
    }
}

#[derive(Clone)]
pub struct Permutation(Box<[usize]>);

pub trait ComposablePermutation: Clone {
    fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str>;

    /// # Safety
    ///
    /// `self`, `b`, and `into` must all be from the same permutation group.
    unsafe fn compose_into(&self, b: &Self, into: &mut Self);

    /// # Safety
    ///
    /// `self` and `b` must both be from the same permutation group.
    unsafe fn compose(&self, b: &Self) -> Self {
        let mut result = self.clone();
        unsafe { self.compose_into(b, &mut result) };
        result
    }
}

impl ComposablePermutation for Permutation {
    fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str> {
        // ... validate that `mapping` is a valid permutation
        Ok(Self(mapping.into_boxed_slice()))
    }

    /// # Safety
    ///
    /// `self`, `b`, and `into` must all be from the same permutation group.
    unsafe fn compose_into(&self, b: &Self, into: &mut Self) {
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }
}
