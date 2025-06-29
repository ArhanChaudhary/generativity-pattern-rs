use std::marker::PhantomData;

pub struct PermGroup<Tok> {
    base_permutation_length: usize,
    base_permutations: Vec<Permutation<Tok>>,
}

impl<Tok> PermGroup<Tok> {
    /// # Safety
    ///
    /// Unsafe public API. Use the `new_perm_group!` macro instead.
    pub unsafe fn new(
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

    pub fn base_permutations(&self) -> &[Permutation<Tok>] {
        &self.base_permutations
    }
}

pub struct Permutation<Tok>(Box<[usize]>, PhantomData<Tok>);

impl<Tok> Permutation<Tok> {
    fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str> {
        // ... validate that `mapping` is a valid permutation
        Ok(Permutation(mapping.into_boxed_slice(), PhantomData))
    }

    pub fn compose_into(&self, b: &Permutation<Tok>, into: &mut Permutation<Tok>) {
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    pub fn compose(&self, b: &Permutation<Tok>) -> Permutation<Tok> {
        let mut result = Permutation(vec![0; self.0.len()].into_boxed_slice(), PhantomData);
        self.compose_into(b, &mut result);
        result
    }
}

#[macro_export]
macro_rules! new_perm_group {
    ($len:expr, $mappings:expr) => {{
        let len = $len;
        let mappings = $mappings;
        struct InvariantToken;
        // SAFETY: private API, only used in this macro
        unsafe {
            $crate::mod_6_unsound_token::PermGroup::<InvariantToken>::new(len, mappings)
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn unsound() {
        let first = (4, vec![vec![1, 2, 0, 3]]);
        let second = (3, vec![vec![2, 0, 1]]);

        let perm_groups = [first, second]
            .into_iter()
            .map(|(len, mappings)| new_perm_group!(len, mappings).unwrap())
            .collect::<Vec<_>>();
        let first_perm = &perm_groups[0].base_permutations()[0];
        let second_perm = &perm_groups[1].base_permutations()[0];

        first_perm.compose(second_perm); // No compile error, UB!
    }
}
