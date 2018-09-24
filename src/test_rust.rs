extern crate fnv;
use self::fnv::FnvHashMap;

use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::time::Instant;

use super::mem_stats::{native_mem_stats_start, native_mem_stats_stop, stats_print};

/// Create a hash set. This benchmark adds data[i] to the hash set if it does
/// not exist, and deletes it if it does.
pub fn insert_or_remove<T: Hash + Eq>(data: &Vec<T>, je_stats: bool) -> usize {
    let now = Instant::now();
    if je_stats {
        stats_print();
    } else {
        // Native mem stats (see native/mem_stats.c)
        // Note: Checkpoints into a static global
        native_mem_stats_start();
    }

    // FnvHashMap::default() is like HashMap::new(), but with a slightly faster
    // hash function for short keys
    let mut h = FnvHashMap::default();

    //for (i, d) in data.iter().enumerate() works but i is then usize (8 bytes)
    for (i, d) in (0..(data.len() as u32)).zip(data) {
        // Entry allows us to perform one key-based operation and then
        // insert/remove based on occupancy
        match h.entry(d) {
            Entry::Occupied(o) => {
                o.remove_entry();
            }
            Entry::Vacant(v) => {
                v.insert(i);
            }
        }
    }

    println!("[benchmark] # elements in hash: {}", h.len());
    if je_stats {
        // now.elapsed().as_micros() would work but u128 us unstable
        let elapsed =
            now.elapsed().as_secs() as f64 + (now.elapsed().subsec_micros() as f64 / 1000000.);
        stats_print();
        println!("CPU time: {}", elapsed);
    } else {
        native_mem_stats_stop();
    }
    return h.len();
}

#[test]
fn test_insert_or_remove() {
    let v = vec!["abc", "def", "ghi", "def"];
    let c = insert_or_remove(&v, false);
    assert_eq!(c, 2);
}

#[test]
fn test_insert_or_remove_int() {
    let v = vec![10, 2, 3, 4, 10, 3, 5];
    let c = insert_or_remove(&v, false);
    assert_eq!(c, 3);
}
