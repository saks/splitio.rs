use crate::condition::{Condition, Partition};
use crate::treatment::Treatment;

// Gets the appropriate treatment based on id, seed and partition value
pub fn get_treatment(id: &str, seed: i64, condition: &Condition) -> Treatment {
    let first_partition = match condition.partitions.first() {
        Some(partition) => partition,
        None => return Treatment::Control,
    };

    if is_hundred_percent_one_treatment(condition, &first_partition) {
        return first_partition.treatment;
    }

    get_treatment_for_key(bucket(count_hash(id, seed)), &condition.partitions)
}

// Returns the treatment for a bucket given the partitions
fn get_treatment_for_key(bucket: u32, partitions: &Vec<Partition>) -> Treatment {
    let mut buckets_covered_thus_far = 0;

    for p in partitions {
        buckets_covered_thus_far += p.size;
        if buckets_covered_thus_far >= bucket {
            return p.treatment;
        }
    }

    Treatment::Control
}

// Returns a hash value for the give key, seed pair
fn count_hash(id: &str, seed: i64) -> u32 {
    let mut x = id.as_bytes().clone();
    murmur3::murmur3_32(&mut x, seed as u32)
}

// Returns bucket value for the given hash value
fn bucket(hash_value: u32) -> u32 {
    (hash_value % 100) + 1
}

// Checks if the partiotion size is 100%
fn is_hundred_percent_one_treatment(condition: &Condition, first_partition: &Partition) -> bool {
    if condition.partitions.len() != 1 {
        false
    } else {
        first_partition.is_hundred_percent()
    }
}
