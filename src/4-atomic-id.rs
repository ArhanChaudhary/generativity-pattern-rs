use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

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
        // ... validate that each mapping is a valid
        // permutation of the given length
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
        let permutation = Permutation::from_mapping(mapping, self.id)?;
        // ... validate that `permutation` is a member of this group
        Ok(permutation)
    }

    pub fn base_permutations(&self) -> &[Permutation] {
        &self.base_permutations
    }
}

pub struct Permutation(Box<[usize]>, u64);

impl Permutation {
    pub(crate) fn from_mapping(mapping: Vec<usize>, id: u64) -> Result<Permutation, &'static str> {
        // ... validate that `mapping` is a valid permutation
        // that is a member of this group
        Ok(Permutation(mapping.into_boxed_slice(), id))
    }

    pub fn compose_into(
        &self,
        b: &Permutation,
        into: &mut Permutation,
    ) -> Result<(), &'static str> {
        if self.1 != b.1 || self.1 != into.1 {
            return Err("Permutations must come from the same group");
        }
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
        Ok(())
    }

    pub fn compose(&self, b: &Permutation) -> Result<Permutation, &'static str> {
        let mut result = Permutation(vec![0; self.0.len()].into_boxed_slice(), self.1);
        self.compose_into(b, &mut result)?;
        Ok(result)
    }
}
