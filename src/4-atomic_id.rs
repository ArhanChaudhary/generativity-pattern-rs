use crate::{validate_permutation, validate_permutation_group_membership};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// Permutation composition within the same permutation group
/// upholds the membership invariant; thus, callers can assume
/// `Permutation::compose` produces another permutation in this
/// permutation group.
pub struct PermGroup {
    base_permutation_length: usize,
    base_permutations: Vec<Permutation>,
    id: u64,
}

static ID: AtomicU64 = AtomicU64::new(0);

impl PermGroup {
    pub fn new(
        base_permutation_length: usize,
        base_permutation_mappings: Vec<Vec<usize>>,
    ) -> Result<Self, &'static str> {
        for mapping in &base_permutation_mappings {
            validate_permutation(mapping, base_permutation_length)?;
        }
        let id = ID.fetch_add(1, Relaxed);
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
    ) -> Result<Permutation, &'static str> {
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

    pub fn base_permutations(&self) -> &[Permutation] {
        &self.base_permutations
    }
}

pub struct Permutation(Box<[usize]>, u64);

impl Permutation {
    /// # Safety
    ///
    /// `Permutation`s with the same `id` must:
    /// - be valid permutations of the same length
    /// - uphold any other defined invariants
    ///
    /// Callers can safely violate this contract as long as the
    /// resulting `Permutation` is never used for composition.
    pub unsafe fn from_mapping(mapping: Vec<usize>, id: u64) -> Result<Self, &'static str> {
        validate_permutation(&mapping, mapping.len())?;
        Ok(Self(mapping.into_boxed_slice(), id))
    }

    /// See the note in `compose`.
    pub fn compose_into(&self, b: &Self, into: &mut Self) -> Result<(), &'static str> {
        if self.1 != b.1 || b.1 != into.1 {
            return Err("Permutations must come from the same permutation group");
        }
        for i in 0..into.0.len() {
            // SAFETY: `self`, `b`, and `into` have the same ID.
            // Therefore, they are valid permutations of the
            // same length that uphold any defined invariants
            // when composed.
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
        Ok(())
    }

    /// Calling code can safely assume permutation composition
    /// upholds the invariants defined in `from_mapping`.
    pub fn compose(&self, b: &Self) -> Result<Self, &'static str> {
        let mut result = Self(vec![0; self.0.len()].into_boxed_slice(), self.1);
        self.compose_into(b, &mut result)?;
        Ok(result)
    }
}
