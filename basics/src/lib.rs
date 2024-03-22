/*
 * https://rust-analyzer.github.io/manual.html#vs-code-2
 * https://algodaily.com/lessons/types-of-sorting-algorithm-cheat-sheet
 */

// use std::cmp::PartialOrd;

/* TODO:
 * There is a LinkedList included in standard collections,
 * But it might be good practice to build own that can be sorted.
 */

mod lists;
mod trees;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod sorting {
    pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
        /*Function to sort in-place, no return value
         * https://www.interviewcake.com/sorting-algorithm-cheat-sheet
         * Selection sort finds the minimum value and puts in current location
         */
        let length = arr.len();

        // (length-1) so we don't unnecessarly check last value, and prevent index error in next loop
        for main_index in 0..length - 1 {
            let mut swap = false;
            let mut swap_index = main_index;

            // (main_index+1) so we don't unnecessarily check against itself.
            for current_index in (main_index + 1)..length {
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

    pub fn insertion_sort<T: PartialOrd>(unsorted: &mut [T]) {
        let length = unsorted.len();
        let mut counter = 0;
        while counter < (length - 1) {
            // If value greater than next, we swap and increase counter.
            if unsorted[counter] > unsorted[counter + 1] {
                unsorted.swap(counter, counter + 1);
                // Ensure counter does not go below zero
                if counter != 0 {
                    counter -= 1;
                } else {
                    counter += 1;
                }
            } else {
                counter += 1;
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
    fn test_int_bubble_sort() {
        use sorting::bubble_sort;
        let mut arr = [5, 2, 9, 1, 5, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_int_vec_bubble_sort() {
        use sorting::bubble_sort;
        let mut arr = vec![90, 5, 2, 9, 1, 5, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 5, 5, 6, 9, 90]);
    }

    #[test]
    fn test_char_bubble_sort() {
        use sorting::bubble_sort;
        let mut arr = ['D', 't', 'T', 'd', 'A', 'i', 'Z'];
        bubble_sort(&mut arr);
        assert_eq!(arr, ['A', 'D', 'T', 'Z', 'd', 'i', 't']);
    }

    #[test]
    fn test_string_bubble_sort() {
        use sorting::bubble_sort;
        let mut arr = ["Hi", "Hello", "Goodbye"];
        bubble_sort(&mut arr);
        assert_eq!(arr, ["Goodbye", "Hello", "Hi"]);
    }

    #[test]
    fn test_int_selection_sort() {
        use sorting::selection_sort;
        let mut arr = [5, 10, 3, 6, 9, 3, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 5, 6, 9, 10]);

        let mut arr2 = [5, 4, 3, 2, 1];
        selection_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);
    }

    // Selection Sort Tests
    #[test]
    fn test_vec_int_selection_sort() {
        use sorting::selection_sort;
        let mut arr = vec![5, 10, 3, 6, 9, 3, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 5, 6, 9, 10]);

        let mut arr2 = [5, 4, 3, 2, 1];
        selection_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_char_selection_sort() {
        use sorting::selection_sort;
        let mut arr = ['C', 'z', 'A', 'a', 'k', 'F'];
        selection_sort(&mut arr);
        assert_eq!(arr, ['A', 'C', 'F', 'a', 'k', 'z']);
    }

    #[test]
    fn test_string_selection_sort() {
        use sorting::selection_sort;
        let mut arr = ["Hi", "Hello", "Goodbye"];
        selection_sort(&mut arr);
        assert_eq!(arr, ["Goodbye", "Hello", "Hi"]);
    }
    // Insertion Sort Tests
    #[test]
    fn test_vec_int_insertion_sort() {
        use sorting::insertion_sort;
        let mut arr = vec![5, 10, 3, 6, 9, 3, 4, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 5, 6, 9, 10]);

        let mut arr2 = [5, 4, 3, 2, 1];
        insertion_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_char_insertion_sort() {
        use sorting::insertion_sort;
        let mut arr = ['C', 'z', 'A', 'a', 'k', 'F'];
        insertion_sort(&mut arr);
        assert_eq!(arr, ['A', 'C', 'F', 'a', 'k', 'z']);
    }

    #[test]
    fn test_string_insertion_sort() {
        use sorting::insertion_sort;
        let mut arr = ["Cherry", "Cherries", "Banana", "Apple"];
        insertion_sort(&mut arr);
        assert_eq!(arr, ["Apple", "Banana", "Cherries", "Cherry"]);
    }
}

/* Why take in a mutable reference?
* The & create an array slice. A slice is, or is like, a smart pointer
* that automatically implements the Deref trait.
*/
