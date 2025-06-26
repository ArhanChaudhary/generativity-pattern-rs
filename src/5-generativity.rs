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
        // ... validate that each mapping is a valid
        // permutation of the given length
        // (remember that permutations can only be composed
        // if they have the same length)
        let id = guard.into();
        Ok(Self {
            base_permutation_length,
            base_permutations: base_permutation_mappings
                .into_iter()
                .map(|mapping| Permutation::from_mapping(mapping, id))
                .collect::<Result<_, _>>()?,
            id,
        })
    }

    pub fn permutation_from_mapping(
        &self,
        mapping: Vec<usize>,
    ) -> Result<Permutation<'id>, &'static str> {
        let permutation = Permutation::from_mapping(mapping, self.id)?;
        // ... validate that `permutation` is a member of this group
        Ok(permutation)
    }

    pub fn base_permutations(&self) -> &[Permutation<'id>] {
        &self.base_permutations
    }
}

pub struct Permutation<'id>(Box<[usize]>, Id<'id>);

impl<'id> Permutation<'id> {
    pub(crate) fn from_mapping(mapping: Vec<usize>, id: Id<'id>) -> Result<Self, &'static str> {
        // ... validate that `mapping` is a valid permutation
        Ok(Permutation(mapping.into_boxed_slice(), id))
    }

    pub fn compose_into(&self, b: &Permutation<'id>, into: &mut Permutation<'id>) {
        for i in 0..into.0.len() {
            unsafe {
                *into.0.get_unchecked_mut(i) = *self.0.get_unchecked(*b.0.get_unchecked(i));
            }
        }
    }

    pub fn compose(&self, b: &Permutation<'id>) -> Permutation<'id> {
        let mut result = Permutation(vec![0; self.0.len()].into_boxed_slice(), self.1);
        self.compose_into(b, &mut result);
        result
    }
}
