pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod sorting {
    pub fn int_bubble_sort(arr: &mut [i32]) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n-i-1 {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_int_bubsort() {
        use sorting::int_bubble_sort;
        let mut arr = [5,2,9,1,5,6];
        int_bubble_sort(&mut arr);
        assert_eq!(arr, [1,2,5,5,6,9]);
    }
}