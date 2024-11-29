// More general purpose binary search algorithms than just operating on containers.

// Returns the first element for which the predicate returns false.
pub fn binary_search(mut start: u64, mut end: u64, mut predicate: impl FnMut(u64) -> bool) -> u64 {
    while end > start {
        let mid = start + (end - start) / 2;

        if predicate(mid) {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start
}

pub fn exponential_search(start: u64, mut predicate: impl FnMut(u64) -> bool) -> u64 {
    let mut step = 1;
    loop {
        if !predicate(start + step) {
            break;
        }

        step *= 2;
    }

    binary_search(start + step / 2, start + step, predicate)
}
