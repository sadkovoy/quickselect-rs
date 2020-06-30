extern crate rand;

use rand::Rng;

fn partition(items: &mut[i32], left: usize, right: usize) -> usize {
    let mut random = rand::thread_rng();

    let mut i = left;

    let pivot_idx = random.gen_range(left, right + 1);
    let pivot = items[pivot_idx];

    items.swap(pivot_idx, right);

    for j in left..=right {
        if items[j] < pivot {
            items.swap(i, j);
            i += 1;
        }
    }

    items.swap(i, right);

    i
}

pub fn quickselect(items: &mut[i32], k: usize) -> i32 {
    let k = k - 1;  // k is 1-based index

    let mut left = 0;
    let mut right = items.len() - 1;

    while left <= right {
        let pivot = partition(items, left, right);
        if pivot == k {
            return items[pivot];
        } else if pivot > k {
            right = pivot - 1;
        } else {
            left = pivot + 1;
        }
    }

    -1
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_k() {
        assert_eq!(quickselect(&mut [3,2,3,1,2,4,5,5,6], 6), 4);
    }
}
