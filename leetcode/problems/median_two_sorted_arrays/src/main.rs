pub fn find_median_sorted_arrays(n1: Vec<i32>, n2: Vec<i32>) -> f64 {
    let half_length = (n1.len() + n2.len() + 1) / 2;
    let l: &Vec<i32>;
    let r: &Vec<i32>;
    // simplify by having 'l' always be atleast shorter
    if n1.len() <= n2.len() {
        l = &n1;
        r = &n2;
    } else {
        l = &n2;
        r = &n1;
    }
    let m = l.len();
    let n = r.len();
    let mut l_bound = (0, l.len()); // initial min/max to 'l' contribution
    while l_bound.0 <= l_bound.1 {
        let i = l_bound.0 + (l_bound.1 - l_bound.0) / 2;
        let j = half_length - i;
        if i > 0 && l[i - 1] > r[j] {
            // l[i] lies to right of median,
            // so decrease l contribution by decreasing the bound max
            l_bound.1 = i - 1;
        } else if i < m && r[j - 1] > l[i] {
            // r[j] lies to left of median,
            // so increase l contribution by increasing the bound min
            l_bound.0 = i + 1;
        } else {
            // have median
            let left_half_end = {
                if i == 0 {
                    r[j - 1]
                } else if j == 0 {
                    l[i - 1]
                } else {
                    l[i - 1].max(r[j - 1])
                }
            };
            if (m + n) & 1 == 0 {
                let right_half_start = {
                    if i == m {
                        r[j]
                    } else if j == n {
                        l[i]
                    } else {
                        l[i].min(r[j])
                    }
                };
                return (left_half_end + right_half_start) as f64 / 2.0;
            } else {
                return left_half_end as f64;
            }
        }
    }
    unreachable!();
}

fn main() {
    let nums1 = vec![4, 20, 32, 50, 55, 61];
    let nums2 = vec![1, 15, 22, 30, 70];
    println!("{}", find_median_sorted_arrays(nums1, nums2));
}

pub fn test(nums1: Vec<i32>, nums2: Vec<i32>, ans: f64) {
    assert_eq!(find_median_sorted_arrays(nums1, nums2), ans);
}

#[test]
fn test_1() {
    test(
        vec![4, 20, 32, 50, 55, 61],
        vec![1, 15, 22, 30, 70],
        30.0);
}
