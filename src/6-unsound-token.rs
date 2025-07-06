use crate::{validate_permutation, validate_permutation_group_membership};
use std::marker::PhantomData;

/// Permutation composition within the same permutation group
/// upholds the membership invariant; thus, callers can assume
/// `Permutation::compose` produces another permutation in this
/// permutation group.
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
        for mapping in &base_permutation_mappings {
            validate_permutation(mapping, base_permutation_length)?;
        }
        Ok(Self {
            base_permutation_length,
            base_permutations: base_permutation_mappings
                .into_iter()
                .map(|mapping| Permutation(mapping.into_boxed_slice(), PhantomData::<Tok>))
                .collect(),
        })
    }

    pub fn permutation_from_mapping(
        &self,
        mapping: Vec<usize>,
    ) -> Result<Permutation<Tok>, &'static str> {
        // SAFETY: the resulting `Permutation` is only used for
        // composition if it is a member of this permutation
        // group.
        let permutation = unsafe { Permutation::from_mapping(mapping)? };
        validate_permutation_group_membership(
            &permutation.0,
            &self
                .base_permutations
                .iter()
                .map(|p| &*p.0)
                .collect::<Vec<_>>(),
        )?;
        Ok(permutation)
    }

    pub fn base_permutations(&self) -> &[Permutation<Tok>] {
        &self.base_permutations
    }
}

pub struct Permutation<Tok>(Box<[usize]>, PhantomData<Tok>);

impl<Tok> Permutation<Tok> {
    /// # Safety
    ///
    /// `Permutation`s with the same `Tok` brand must:
    /// - be valid permutations of the same length
    /// - uphold any other defined invariants
    ///
    /// Callers can safely violate this contract as long as the
    /// resulting `Permutation` is never used for composition.
    pub unsafe fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str> {
        validate_permutation(&mapping, mapping.len())?;
        Ok(Self(mapping.into_boxed_slice(), PhantomData))
    }

    /// See the note in `compose`.
    pub fn compose_into(&self, b: &Permutation<Tok>, into: &mut Permutation<Tok>) {
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    /// Calling code can safely assume permutation composition
    /// upholds the invariants defined in `from_mapping`.
    pub fn compose(&self, b: &Permutation<Tok>) -> Permutation<Tok> {
        let mut result = Self(vec![0; self.0.len()].into_boxed_slice(), PhantomData);
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
        // SAFETY: private API, only used in this macro.
        unsafe { $crate::mod_6_unsound_token::PermGroup::<InvariantToken>::new(len, mappings) }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn unsound() {
        let first = (4, vec![vec![1, 2, 0, 3]]);
        let second = (3, vec![vec![2, 0, 1]]);

        let mut perm_groups = vec![];
        for (len, mappings) in [first, second] {
            perm_groups.push(new_perm_group!(len, mappings).unwrap());
        }
        let first_perm = &perm_groups[0].base_permutations()[0];
        let second_perm = &perm_groups[1].base_permutations()[0];

        first_perm.compose(second_perm); // No compile error, UB!
    }
}
