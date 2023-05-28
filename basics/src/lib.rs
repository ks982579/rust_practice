/*
 * https://rust-analyzer.github.io/manual.html#vs-code-2
 */

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

    pub fn int_selection_sort(arr: &mut [i32]) {
        /*Function to sort in-place, no return value 
        * https://www.interviewcake.com/sorting-algorithm-cheat-sheet
        * Selection sort finds the minimum value and puts in current location
        */
        let length = arr.len();
        
        // (length-1) so we don't unnecessarly check last value, and prevent index error in next loop
        for main_index in 0..length-1 {
            let mut swap = false;
            let mut swap_index = main_index;

            // (main_index+1) so we don't unnecessarily check against itself.
            for current_index in (main_index+1)..length {
                if arr[current_index] < arr[swap_index] {
                    swap = true;
                    swap_index = current_index;
                }
            }
            if swap {
                arr.swap(main_index, swap_index);
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

    #[test]
    fn test_int_selection_sort() {
        use sorting::int_selection_sort;
        let mut arr = [5,10,3,6,9,3,4,2];
        int_selection_sort(&mut arr);
        assert_eq!(arr, [2,3,3,4,5,6,9,10]);

        let mut arr2 = [5,4,3,2,1];
        int_selection_sort(&mut arr2);
        assert_eq!(arr2, [1,2,3,4,5]);
    }
}

/* Why take in a mutable reference?
* The & create an array slice. A slice is, or is like, a smart pointer
* that automatically implements the Deref trait. 
*/