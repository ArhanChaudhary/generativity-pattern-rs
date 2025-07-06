use crate::{validate_permutation, validate_permutation_group_membership};
use generativity::{Guard, Id};

/// Permutation composition within the same permutation group
/// upholds the membership invariant; thus, callers can assume
/// `Permutation::compose` produces another permutation in this
/// permutation group.
pub struct PermGroup<'id> {
    base_permutation_length: usize,
    base_permutations: Vec<Permutation<'id>>,
    id: Id<'id>,
}

impl<'id> PermGroup<'id> {
    pub fn new(
        base_permutation_length: usize,
        base_permutation_mappings: Vec<Vec<usize>>,
        guard: Guard<'id>,
    ) -> Result<Self, &'static str> {
        for mapping in &base_permutation_mappings {
            validate_permutation(mapping)?;
        }
        let id = guard.into();
        Ok(Self {
            base_permutation_length,
            base_permutations: base_permutation_mappings
                .into_iter()
                .map(|mapping| Permutation(mapping.into_boxed_slice(), id))
                .collect(),
            id,
        })
    }

    pub fn permutation_from_mapping(
        &self,
        mapping: Vec<usize>,
    ) -> Result<Permutation<'id>, &'static str> {
        // SAFETY: the resulting `Permutation` is only used for
        // composition if it is a member of this permutation
        // group.
        let permutation = unsafe { Permutation::from_mapping(mapping, self.id)? };
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

    pub fn base_permutations(&self) -> &[Permutation<'id>] {
        &self.base_permutations
    }
}

pub struct Permutation<'id>(Box<[usize]>, Id<'id>);

impl<'id> Permutation<'id> {
    /// # Safety
    ///
    /// `Permutation`s with the same `'id` brand must:
    /// - be valid permutations of the same length
    /// - uphold any other defined invariants
    ///
    /// Callers can safely violate this contract as long as the
    /// resulting `Permutation` is never used for composition.
    pub unsafe fn from_mapping(mapping: Vec<usize>, id: Id<'id>) -> Result<Self, &'static str> {
        validate_permutation(&mapping)?;
        Ok(Self(mapping.into_boxed_slice(), id))
    }

    /// See the note in `compose`.
    pub fn compose_into(&self, b: &Self, into: &mut Self) {
        for i in 0..into.0.len() {
            // SAFETY: `self`, `b`, and `into` have the same
            // lifetime brand. Therefore, they are valid
            // permutations of the same length that uphold any
            // defined invariants when composed.
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    /// Calling code can safely assume permutation composition
    /// upholds the invariants defined in `from_mapping`.
    pub fn compose(&self, b: &Self) -> Self {
        let mut result = Self(vec![0; self.0.len()].into_boxed_slice(), self.1);
        self.compose_into(b, &mut result);
        result
    }
}
