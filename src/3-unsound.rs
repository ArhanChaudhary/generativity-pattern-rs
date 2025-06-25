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

pub struct Permutation(Box<[usize]>);

impl Permutation {
    pub(crate) fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str> {
        // ... validate that `mapping` is a valid permutation
        Ok(Permutation(mapping.into_boxed_slice()))
    }

    pub fn compose_into(&self, b: &Permutation, into: &mut Permutation) {
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    pub fn compose(&self, b: &Permutation) -> Permutation {
        let mut result = Permutation(vec![0; self.0.len()].into_boxed_slice());
        self.compose_into(b, &mut result);
        result
    }
}
