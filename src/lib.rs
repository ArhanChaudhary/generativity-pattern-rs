#[path = "1-slice.rs"]
pub mod mod_1_slice;
#[path = "2-newtype.rs"]
pub mod mod_2_newtype;
#[path = "3-unsafe-trait.rs"]
pub mod mod_3_unsafe_trait;
#[path = "4-atomic-id.rs"]
pub mod mod_4_atomic_id;
#[path = "5-generativity.rs"]
pub mod mod_5_generativity;
#[path = "6-unsound-token.rs"]
pub mod mod_6_unsound_token;

fn validate_permutation(mapping: &[usize]) -> Result<(), &'static str> {
    let length = mapping.len();
    // TODO
    if mapping.len() != length {
        return Err("Mapping length does not match the expected permutation length");
    }
    let mut seen = vec![false; length];
    for &index in mapping {
        if index >= length {
            return Err("Mapping contains an index out of bounds");
        }
        if seen[index] {
            return Err("Mapping contains duplicate indices");
        }
        seen[index] = true;
    }
    Ok(())
}

fn validate_permutation_group_membership(
    permutation: &[usize],
    base_permutations: &[&[usize]],
) -> Result<(), &'static str> {
    // Uses the [Schreier-Sims algorithm](https://en.wikipedia.org/wiki/Schreier%E2%80%93Sims_algorithm)
    let permutation2 = permutation_rs::group::permutation::Permutation::new(
        permutation
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as u64, value as u64))
            .collect(),
    );

    let gset = (0..base_permutations
        .iter()
        .flat_map(|g| g.iter())
        .map(|&x| x as u64)
        .max()
        .unwrap_or(0))
        .collect();
    let generators = base_permutations
        .iter()
        .map(|g| {
            permutation_rs::group::permutation::Permutation::new(
                g.iter()
                    .enumerate()
                    .map(|(i, &value)| (i as u64, value as u64))
                    .collect(),
            )
        })
        .collect();

    let group = permutation_rs::group::Group::new(gset, generators);
    if group.is_member(permutation2) {
        Ok(())
    } else {
        Err("Permutation is not a member of this group")
    }
}
