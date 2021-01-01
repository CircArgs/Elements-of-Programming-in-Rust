//! ### The Dutch National Flag Problem
//! Write a program that takes an array slice and an index i into slice, and rearranges the elements such
//! that all elements less than slice[r] (the "pivot") appear first, followed by elements equal to the pivot,
//! followed by elements greater than the pivot.
//! Hinf: Think about the partition step in quicksort.

//! [the documentation for the slice primitive comes in handy](https://doc.rust-lang.org/std/primitive.slice.html)

/// the solution keeps track of two pivot points to know the "middle stripe" start and end
/// starting from the beginning it moves larger elements to the end and equal elements to the middle stripe
/// then works on the back half to move smaller elements to the beginning
/// all while making sure not to disturb remaining elements in the start or end loops.
/// This solution has time complexity per slice of O(n^2) because each time we fine an element that is not in the appropriate half
/// we do a rotate which will shift all n elements of the slice - hence 'naive'
fn flag_sort_naive<T: Ord>(slice: &mut [T], pivot: usize) {
    // base case if the slice is 0 or 1 elements it is already arranged
    if slice.len() < 2 {
        return;
    }
    // pivot cannot be less than 0 or more than the len-1
    let pivot = pivot.min(slice.len() - 1);
    //start and end will track the middle elements equal to the pivot
    let (mut start, mut end) = (pivot, pivot);
    //start at the beginning and will do up to the start (exclusive)
    let mut i = 0;
    while i < start {
        // element is greated than the pivot value and belongs after the end pivot
        if slice[i] > slice[start] {
            // swap with the first element preserving all elements before start staying before start
            slice.swap(i, 0);
            // and rotate left so that the element now at the beginning moves to the end of the slice
            slice.rotate_left(1);
            // rotate left shifted the start and end down one
            start -= 1;
            end -= 1;
        }
        // element equals the pivot value then it belongs as part of the stripe
        else if slice[i] == slice[start] {
            // put it next to current start pivot
            slice.swap(i, start - 1);
            // start pivot now is this element after the swap
            start -= 1;
        }
        // unless we moved the current value we do not need to increment the current index
        // because the current element was otherwise moved to the end and the next value shifted forward
        else {
            i += 1;
        }
    }
    // now that all elements from the beginning up to the start have been properly arranged
    // we will work our way from after the end pivot to the end of the slice
    i = end + 1;
    while i < slice.len() {
        // element belongs before start
        if slice[i] < slice[start] {
            // move it to the end (to preserve other elements being located after end)
            slice.swap(i, slice.len() - 1);
            // rotate the slice so the element which is now at the end of the slice moves to the beggining
            slice.rotate_right(1);

            // start and end are now 1 futher back
            start += 1;
            end += 1;
        }
        // element equals pivot value so becomes part of middle "stripe"
        else if slice[i] == slice[start] {
            // we put the element after the element after the end pivot (extend the "stripe")
            slice.swap(i, end + 1);
            // and end pivot is now the location of this element
            end += 1;
        }
        // regardless of what we do in this loop the index goes forward
        i += 1;
    }
    // recurse over lower "stripe" and upper "stripe"
    flag_sort_naive(&mut slice[..start], pivot / 2);
    flag_sort_naive(&mut slice[(end + 1)..], pivot / 2);
}

/// sorts a slice of elements implementing Ord
/// iterates through the slice twice - forwards moving elements less than the pivot to the front
/// - backwards moving elements greater than the pivot to the end of the slice.
/// the position of the pivot must be tracked in order to have the pivot value to compare to without having T implement Copy
/// also for the recursive subslicing to work excluding elements equal to the pivot value
/// unlike `flag_sort_naive` the partitioning takes O(n) time since it iterated through the array twice once from the beginning and once from the rear (and two more short iterations from the final pivot location)
fn flag_sort<T: Ord>(slice: &mut [T], pivot: usize) {
    // base case if the slice is 0 or 1 elements it is already arranged
    if slice.len() < 2 {
        return;
    }
    // pivot cannot be less than 0 or more than the len-1
    let pivot = pivot.min(slice.len() - 1);
    // count will track how many elements have been moved to the beginning of the slice
    // it will be used to tell which value can be swapped with when the next value less than the pivot is found
    let mut count = 0;
    // start represents the position of the pivot as it is potentially moved around by the swapping of elements
    let mut start = pivot;
    // the forward pass moving all elements less than the pivot to the beginning of the slice
    for i in 0..slice.len() {
        // an element less than the slice will be swapped with the element at current `count`
        if slice[i] < slice[start] {
            // we have to check if the element we will be swapping with at `count` is the current position of the pivot i.e. `start`
            // if it is we need to account for the start value now being moved to position `i`
            if count == start {
                start = i;
            }
            // then we perform the swap and increment `count` so that we know we have put another element in the proper place from the start of the slice
            slice.swap(i, count);
            count += 1;
        }
    }
    // now we will be putting elements greater than the pivot from the rear of the slice
    // so count will start with the last element
    count = slice.len() - 1;
    // we perform the backwards pass
    for i in (0..slice.len()).rev() {
        // for the backwards pass we already know we have put smalled elements towards the beginning of the slice so if we see one we know we will no longer encounter elements greater than the pivot
        if slice[i] < slice[start] {
            break;
        }
        if slice[i] > slice[start] {
            // we have to check if the element we will be swapping with at `count` is the current position of the pivot i.e. `start`
            // if it is we need to account for the start value now being moved to position `i`
            if count == start {
                start = i;
            }
            // then we perform the swap and decrement count since we are moving elements to the end of the slice
            slice.swap(i, count);
            count -= 1;
        }
    }

    // at this point all elements less than the pivot form a contiguous region from the start of the slice up to a middle region of values equal to the pivot value
    // followed by a contiguos region of elements greater than the pivot
    // so now we just need to take account of which indices are equal to the pivot i.e. find the middle region of elements equal to the pivot which we will not need to recurse upon
    let mut left = 0;
    let mut right = 0;
    // elements to the left of `start` that are pivot values
    for i in (0..start).rev() {
        if slice[i] == slice[start] {
            left += 1;
        }
    }
    // elements to the right of `start` that are pivot values
    for i in (start + 1)..slice.len() {
        if slice[i] == slice[start] {
            right += 1;
        }
    }
    // now simply the recursive step performing the partitioning of this function the right and left halves respectively
    // and again recursively until the slice is sorted
    flag_sort(&mut slice[0..(start - left)], pivot / 2);
    flag_sort(&mut slice[(start + right + 1)..], pivot / 2);
}

fn flag_part_naive<T: Ord + Clone>(slice: &mut [T], pivot: usize) {
    let pivot = pivot.min(slice.len() - 1);
    let pivot_val = slice[pivot].clone();
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            if slice[j] < pivot_val {
                slice.swap(i, j);
                break;
            }
        }
    }
    for i in (0..slice.len()).rev() {
        if slice[i] < pivot_val {
            break;
        };
        for j in (0..i).rev() {
            if slice[j] > pivot_val {
                slice.swap(i, j);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flag_sort() {
        let tests = vec![
            vec![1, 2, 3],
            vec![8, 6, 7, 8, 5, 5, 6, 5, 3, 0, 9],
            vec![0, 2],
            vec![3, 0],
            vec![1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        for p in 0..15 {
            for t in &tests {
                let mut sol = t.clone();
                sol.sort();
                let mut things1 = t.clone();
                let mut things2 = t.clone();
                flag_sort_naive(&mut things1, p);
                assert_eq!(things1, sol);
                flag_sort(&mut things2, p);
                assert_eq!(things2, sol);
            }
        }
    }

    #[test]
    fn test_flag_part_naive() {
        let mut things = vec![8, 6, 7, 5, 5, 6, 5, 3, 0, 9];
        flag_part_naive(&mut things, 3);
        assert_eq!(things, &[3, 0, 5, 5, 5, 7, 9, 6, 8, 6]);
    }
}
