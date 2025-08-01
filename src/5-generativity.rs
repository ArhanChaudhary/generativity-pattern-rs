use crate::{validate_permutation, validate_permutation_group_membership};
use generativity::{Guard, Id};

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
            validate_permutation(mapping, base_permutation_length)?;
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

    pub fn base_permutations(&self) -> &[Permutation<'id>] {
        &self.base_permutations
    }
}

pub struct Permutation<'id>(Box<[usize]>, Id<'id>);

impl<'id> Permutation<'id> {
    pub fn from_mapping_and_group(
        mapping: Vec<usize>,
        group: &PermGroup<'id>,
    ) -> Result<Self, &'static str> {
        validate_permutation(&mapping, group.base_permutation_length)?;
        let permutation = Self(mapping.into_boxed_slice(), group.id);
        validate_permutation_group_membership(
            &permutation.0,
            &group
                .base_permutations
                .iter()
                .map(|p| &*p.0)
                .collect::<Vec<_>>(),
        )?;
        Ok(permutation)
    }

    pub fn compose_into(&self, b: &Self, result: &mut Self) {
        for i in 0..result.0.len() {
            // SAFETY: `self`, `b`, and `into` have the same
            // lifetime brand. Therefore, they are valid
            // permutations of the same length that uphold any
            // defined invariants when composed.
            unsafe {
                *result.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    pub fn compose(&self, b: &Self) -> Self {
        let mut result = Self(vec![0; self.0.len()].into_boxed_slice(), self.1);
        self.compose_into(b, &mut result);
        result
    }
}
