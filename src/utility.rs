use std::collections::HashSet;



pub fn repeat_list<T: Clone>(input: &[T], n: usize) -> Vec<T> {
    let mut repeated_list = Vec::with_capacity(input.len() * n);
    
    for _ in 0..n {
        repeated_list.extend_from_slice(input);
    }
    
    repeated_list
}

pub fn stretch_list<T: Clone>(input: &[T], n: usize) -> Vec<T> {
    let mut stretched_list = Vec::with_capacity(input.len() * n);
    
    for item in input {
        for _ in 0..n {
            stretched_list.push(item.clone());
        }
    }
    
    stretched_list
}

pub fn sort_vector<T: PartialOrd + std::clone::Clone>(slice: &[T]) -> Vec<T> {
    let mut sorted_vec: Vec<T> = slice.to_vec();
    sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_vec
}

pub fn vector_to_hashset<T: Eq + std::hash::Hash + Clone>(arr: &[T]) -> HashSet<T> {
    let set: HashSet<_> = arr.iter().cloned().collect();
    set
}

pub fn cyclically_order_vector<T: Ord + std::clone::Clone>(slice: &[T], start: T) -> Vec<T>{
    let start_index = slice.iter().position(|x| *x == start).unwrap();
    let mut cyclically_ordered: Vec<T> = slice.to_vec();
    cyclically_ordered.rotate_left(start_index);
    cyclically_ordered
}

pub fn cyclically_order_floats(floats: &[f64], start: f64) -> Vec<f64>{
    let start_index = floats.iter().position(|x| *x == start).unwrap();
    let mut cyclically_ordered: Vec<f64> = floats.to_vec();
    cyclically_ordered.rotate_left(start_index);
    cyclically_ordered
}

pub fn find_aperiodic_substring<T: PartialEq + Clone>(sequence: &[T]) -> Vec<T> {
    let sequence_len = sequence.len();

    #[warn(clippy::unwrap_or_else_default)]
    (1..=sequence_len)
        .filter(|&block_size| sequence_len % block_size == 0)
        .find_map(|block_size| {
            let num_blocks = sequence_len / block_size;
            let blocks = (0..num_blocks)
                .map(|i| {
                    let start = i * block_size;
                    let end = start + block_size;
                    &sequence[start..end]
                })
                .collect::<Vec<_>>();

            if blocks.iter().all(|&block| block == blocks[0]) {
                Some(blocks[0].to_vec())
            } else {
                None
            }
        }).unwrap_or_default()
}

pub fn hashset_to_vector<T: Ord + Clone>(set: &HashSet<T>) -> Vec<T> {
    let mut sorted_vec: Vec<T> = set.iter().cloned().collect();
    sorted_vec.sort();
    sorted_vec
}


// Checks

pub fn collection_is_unique<T: Eq + std::hash::Hash>(collection: &[T]) -> bool {
    let set: HashSet<_> = collection.iter().collect();
    collection.len() == set.len()
}

pub fn collection_is_sorted<T: Ord>(collection: &[T]) -> bool {
    collection.windows(2).all(|pair: &[T]| pair[0] <= pair[1])
}

pub fn collection_is_cyclically_ascending<T: Ord>(collection: &[T]) -> bool {
    collection.windows(2)
        .all(|pair: &[T]| pair[0] <= pair[1])
        || collection.last() <= collection.first()
}

pub fn floats_are_unique(floats: &[f64]) -> bool {
    let mut unique_floats = floats.to_vec();
    unique_floats.dedup();
    floats.len() == unique_floats.len()
}

pub fn floats_are_sorted(floats: &[f64]) -> bool {
    let mut sorted_floats = floats.to_vec();
    sorted_floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    floats == sorted_floats        
}
