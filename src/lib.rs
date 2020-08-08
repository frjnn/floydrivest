//! floydrivest
//!
//! `floydrivest` is a small and extremely lightweight crate that provides
//! an in-place implementation of the Floyd-Rivest algorithm.
use std::cmp::max;
use std::cmp::min;
/// Moves the n-th element of the given Vector in the n-th position
/// by using the Floyd-Rivest algorithm with linear time complexity.
///
/// Similar to its c++ counterpart.
///
///
/// # Examples
///
/// ```
/// let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4]; // a vector of i64.
/// let len = v.len();
/// floydrivest::nth_element(&mut v, 0, len-1, 3);
///
/// assert_eq!(v[3], 7);
/// ```
///
/// # Panics
///
/// if `left`, `right` or `nth_el` are out of bounds.
pub fn nth_element<T>(a: &mut Vec<T>, mut left: usize, mut right: usize, nth_el: usize)
where
    T: PartialEq + PartialOrd + Clone,
{
    let mut i: usize;
    let mut j: usize;
    let mut t: T;
    while right > left {
        if right - left > 600 {
            // Use recursion on a sample of size s to get an estimate
            // for the (nth_el - left + 1 )-th smallest elementh into a[nth_el],
            // biased slightly so that the (nth_el - left + 1)-th element is expected
            // to lie in the smallest set after partitioning.
            let n: usize = right - left + 1;
            let i: usize = nth_el - left + 1;
            let z: f64 = (n as f64).ln();
            let s: f64 = 0.5 * (2.0 * z / 3.0).exp();
            let sd: f64 =
                0.5 * (z * s * (n as f64 - s) / n as f64).sqrt() * ((i - n) as f64 / 2.0).signum();
            let ll: usize = max(left, nth_el - (i as f64 * s / n as f64 + sd) as usize);
            let rr: usize = min(
                right,
                nth_el + sd as usize + ((n - i) as f64 * s / (n as f64)) as usize,
            );
            nth_element(a, ll, rr, nth_el);
        }
        // The following code partitions a[l : r] about t, it is similar to Hoare's
        // algorith but it'll run faster on most machines since the subscript range
        // checking on i and j has been removed.
        t = a[nth_el].clone();
        i = left;
        j = right;
        a.swap(left, nth_el);
        if a[right] > t {
            a.swap(right, left);
        }
        while i < j {
            a.swap(i, j);
            i += 1;
            j -= 1;
            while a[i] < t {
                i += 1;
            }
            while a[j] > t {
                j -= 1;
            }
        }
        if a[left] == t {
            a.swap(left, j);
        } else {
            j += 1;
            a.swap(j, right);
        }
        // Now we adjust left and right so that they
        // surround the subset containing the
        // (k - left + 1)-th smallest element.
        if j <= nth_el {
            left = j + 1;
        }
        if nth_el <= j {
            right = j - 1;
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::nth_element;
    #[test]
    fn test_simple() {
        let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4];
        let len = v.len();
        nth_element(&mut v, 0, len - 1, 3);
        assert_eq!(v[3], 7);
    }
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_iter() {
        for n in 0..10 {
            let mut v = vec![9, 5, 0, 6, 8, 2, 3, 7, 1, 4];
            let len = v.len();
            nth_element(&mut v, 0, len - 1, n);
            assert_eq!(v[n], n);
        }
    }
}
