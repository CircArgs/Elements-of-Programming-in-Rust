//! Tns Dutch National Flag Problem
//! Write a program that takes an array A and an index i into A, and rearranges the elements such
//! that all elements less than A[r] (the "pivot") appear first, followed by elements equal to the pivot,
//! followed by elements greater than the pivot.
//! Hinf: Think about the partition step in quicksort.

//! [the documentation for the slice primitive comes in handy](https://doc.rust-lang.org/std/primitive.slice.html)

/// the solution keeps track of two pivot points to know the "middle stripe" start and end
/// starting from the beginning it moves larger elements to the end and equal elements to the middle stripe
/// then works on the back half to move smaller elements to the beginning
/// all while making sure not to disturb remaining elements in the start or end loops
fn flag_sort<T: Ord + std::fmt::Debug>(slice: &mut [T], pivot: usize){
    // base case if the slice is 0 or 1 elements it is already arranged
    if slice.len()<2{return;}
    // pivot cannot be less than 0 or more than the len-1
    let pivot = pivot.max(0).min(slice.len()-1);
    //start and end will track the middle elements equal to the pivot
    let (mut start, mut end) = (pivot, pivot);
    //start at the beginning and will do up to the start (exclusive)
    let mut i=0;
    while i<start{
        // element is greated than the pivot value and belongs after the end pivot
        if slice[i]>slice[start]{
            // swap with the first element preserving all elements before start staying before start
            slice.swap(i, 0);
            // and rotate left so that the element now at the beginning moves to the end of the slice
            slice.rotate_left(1);
            // rotate left shifted the start and end down one
            start-=1;
            end-=1;
        }
        // element equals the pivot value then it belongs as part of the stripe
        else if slice[i]==slice[start]{
            // put it next to current start pivot
            slice.swap(i, start-1);
            // start pivot now is this element after the swap
            start-=1;
        }
        // unless we moved the current value we do not need to increment the current index
        // because the current element was otherwise moved to the end and the next value shifted forward
        else{i+=1;}
        
    }
    // now that all elements from the beginning up to the start have been properly arranged
    // we will work our way from after the end pivot to the end of the slice
    i=end+1;
    while i<slice.len(){
        // element belongs before start
        if slice[i]<slice[start]{
            // move it to the end (to preserve other elements being located after end)
            slice.swap(i, slice.len()-1);
            // rotate the slice so the element which is now at the end of the slice moves to the beggining
            slice.rotate_right(1);
            
            // start and end are now 1 futher back
            start+=1;
            end+=1;

        }
        // element equals pivot value so becomes part of middle "stripe"
        else if slice[i]==slice[start]{
            // we put the element after the element after the end pivot (extend the "stripe")
            slice.swap(i, end+1);
            // and end pivot is now the location of this element
            end+=1;
        }
        // regardless of what we do in this loop the index goes forward
        i+=1;
        
    }
    // recurse over lower "stripe" and upper "stripe"
    flag_sort(&mut slice[..start], pivot);
    flag_sort(&mut slice[(end+1)..], pivot);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flag_sort() {
        let mut things = vec![8, 6, 7, 5, 3, 0, 9];
        flag_sort(&mut things, 1);
        assert_eq!(things, &[0, 3, 5, 6, 7, 8, 9]);
    }
}