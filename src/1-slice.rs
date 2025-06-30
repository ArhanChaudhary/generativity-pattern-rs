// We provide a `compose_into` function in case the caller already has
// a permutation preallocated. (This is good practice IMO).
pub fn compose_into(
    a: &[usize],
    b: &[usize],
    into: &mut [usize],
    seen_a: &mut [bool],
    seen_b: &mut [bool],
) -> Result<(), &'static str> {
    if a.len() != b.len() || b.len() != into.len() {
        return Err("Permutations must have the same length");
    }
    seen_a.fill(false);
    seen_b.fill(false);
    for (into_value, &b_value) in into.iter_mut().zip(b) {
        if *seen_b
            .get(b_value)
            .ok_or("B contains an element greater than the length")?
        {
            return Err("B contains duplicate elements");
        }
        seen_b[b_value] = true;

        let a_value = a[b_value];
        if *seen_a
            .get(a_value)
            .ok_or("A contains an element greater than the length")?
        {
            return Err("A contains duplicate elements");
        }
        seen_a[a_value] = true;

        *into_value = a_value;
    }
    Ok(())
}
