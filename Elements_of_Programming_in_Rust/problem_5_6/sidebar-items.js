initSidebarItems({"fn":[["buy_sell","the primary solution takes a slice `&[T]` and returns a T where references to T can be subtracted (`impl Sub for &T {type Output = T}`) and are orderable (`impl Ord for T`) the solution iterates through the slice once and for each element iterates through the remainder of the slice for each element of the inner loop it find the difference which is the amount gained from selling if a better result is found than what is currently had then that becomes the new solution"]]});