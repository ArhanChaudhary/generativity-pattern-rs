#![feature(super_let)]

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
/// This module is unsound!
pub mod mod_6_unsound_token;

pub mod generative_lifetimes;

fn validate_permutation(mapping: &[usize]) -> Result<(), &'static str> {
    let mut seen = vec![false; mapping.len()];
    for &index in mapping {
        if *seen
            .get(index)
            .ok_or("Permutation contains an element greater than the length")?
        {
            return Err("Permutation contains duplicate elements");
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
