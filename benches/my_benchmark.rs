use criterion::{Criterion, criterion_group, criterion_main};
use generativity::make_guard;
use generativity_pattern_rs::{mod_3_unsafe_trait::ComposablePermutation, *};
use std::hint::black_box;

const PERM_A: [usize; 15] = [8, 1, 10, 0, 9, 4, 12, 2, 6, 3, 7, 14, 11, 5, 13];
const PERM_B: [usize; 15] = [1, 4, 7, 3, 10, 12, 6, 8, 5, 13, 0, 14, 2, 11, 9];

pub fn bench_compose_permutations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compose permutations");

    group.bench_function("1-slices", |b| {
        let mut into = [0; 15];
        b.iter(|| {
            mod_1_slice::compose_into(black_box(&PERM_A), black_box(&PERM_B), black_box(&mut into))
        })
    });
    group.bench_function("2-newtype", |b| {
        let perm_a = mod_2_newtype::Permutation::from_mapping(PERM_A.to_vec()).unwrap();
        let perm_b = mod_2_newtype::Permutation::from_mapping(PERM_B.to_vec()).unwrap();
        let mut into = perm_a.compose(&perm_b).unwrap();
        b.iter(|| black_box(&perm_a).compose_into(black_box(&perm_b), black_box(&mut into)))
    });
    group.bench_function("3-unsafe-trait", |b| {
        let perm_group =
            mod_3_unsafe_trait::PermGroup::new(15, vec![PERM_A.to_vec(), PERM_B.to_vec()]).unwrap();
        let perm_a = &perm_group.base_permutations()[0];
        let perm_b = &perm_group.base_permutations()[1];
        let mut into = unsafe { perm_a.compose(perm_b) };
        b.iter(|| unsafe {
            black_box(perm_a).compose_into(black_box(perm_b), black_box(&mut into))
        })
    });
    group.bench_function("4-atomic-id", |b| {
        let perm_group =
            mod_4_atomic_id::PermGroup::new(15, vec![PERM_A.to_vec(), PERM_B.to_vec()]).unwrap();
        let perm_a = &perm_group.base_permutations()[0];
        let perm_b = &perm_group.base_permutations()[1];
        let mut into = perm_a.compose(perm_b).unwrap();
        b.iter(|| black_box(perm_a).compose_into(black_box(perm_b), black_box(&mut into)))
    });
    group.bench_function("5-generativity", |b| {
        make_guard!(guard);
        let perm_group =
            mod_5_generativity::PermGroup::new(15, vec![PERM_A.to_vec(), PERM_B.to_vec()], guard)
                .unwrap();
        let perm_a = &perm_group.base_permutations()[0];
        let perm_b = &perm_group.base_permutations()[1];
        let mut into = perm_a.compose(perm_b);
        b.iter(|| black_box(perm_a).compose_into(black_box(perm_b), black_box(&mut into)))
    });
    group.bench_function("6-unsound-token", |b| {
        let perm_group =
            generativity_pattern_rs::new_perm_group!(15, vec![PERM_A.to_vec(), PERM_B.to_vec()])
                .unwrap();
        let perm_a = &perm_group.base_permutations()[0];
        let perm_b = &perm_group.base_permutations()[1];
        let mut into = perm_a.compose(perm_b);
        b.iter(|| black_box(perm_a).compose_into(black_box(perm_b), black_box(&mut into)))
    });

    group.finish();
}

criterion_group!(benches, bench_compose_permutations);
criterion_main!(benches);
