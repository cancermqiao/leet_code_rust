use std::fmt::Debug;

/// 快速排序
pub fn quick_sort<T: PartialOrd + Clone + Debug>(arr: &mut Vec<T>) {
    _quick_sort(arr, 0, arr.len() - 1)
}

fn _quick_sort<T: PartialOrd + Clone + Debug>(arr: &mut Vec<T>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let (pivot, mut l, mut r) = (left, left, right);
    while l < r {
        while l < r && arr[r] >= arr[pivot] {
            r -= 1;
        }
        while l < r && arr[l] <= arr[pivot] {
            l += 1;
        }
        arr.swap(l, r);
    }
    arr.swap(pivot, l);
    if left + 1 < l {
        _quick_sort(arr, left, l - 1);
    }
    if l + 1 < right {
        _quick_sort(arr, l + 1, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut int_arr = vec![3, 2, 1];
        quick_sort(&mut int_arr);
        assert_eq!(int_arr, vec![1, 2, 3]);

        let mut f64_arr = vec![3.3, 2.71, 1.618];
        quick_sort(&mut f64_arr);
        assert_eq!(f64_arr, vec![1.618, 2.71, 3.3]);
    }
}
