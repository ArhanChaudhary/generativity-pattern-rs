// We provide a `compose_into` function in case the caller already has
// a permutation preallocated. (This is good practice IMO).
pub fn compose_into(a: &[usize], b: &[usize], into: &mut [usize]) -> Result<(), &'static str> {
    if a.len() != b.len() || b.len() != into.len() {
        return Err("Permutations must have the same length");
    }
    let mut seen_b = vec![false; a.len()];
    let mut seen_a = vec![false; b.len()];
    for (into_value, &b_value) in into.iter_mut().zip(b) {
        if *seen_b
            .get(b_value)
            .ok_or("B contains an element greater than the length")?
        {
            return Err("B contains repeated elements");
        }
        seen_b[b_value] = true;

        let a_value = a[b_value];
        if *seen_a
            .get(a_value)
            .ok_or("A contains an element greater than the length")?
        {
            return Err("A contains repeated elements");
        }
        seen_a[a_value] = true;

        *into_value = a_value;
    }
    Ok(())
}
