use crate::validate_permutation;

pub struct Permutation(Box<[usize]>);

impl Permutation {
    pub fn from_mapping(mapping: Vec<usize>) -> Result<Self, &'static str> {
        validate_permutation(&mapping, mapping.len())?;
        Ok(Self(mapping.into_boxed_slice()))
    }

    pub fn compose_into(&self, b: &Self, result: &mut Self) -> Result<(), &'static str> {
        if self.0.len() != b.0.len() || b.0.len() != result.0.len() {
            return Err("Permutations must have the same length");
        }
        for (result_value, &b_value) in result.0.iter_mut().zip(&b.0) {
            // SAFETY: `b` is guaranteed to be a valid permutation
            // whose elements can index `self`
            *result_value = unsafe { *self.0.get_unchecked(b_value) };
        }
        Ok(())
    }

    pub fn compose(&self, b: &Self) -> Result<Self, &'static str> {
        let mut result = Self(vec![0; self.0.len()].into_boxed_slice());
        self.compose_into(b, &mut result)?;
        Ok(result)
    }
}
