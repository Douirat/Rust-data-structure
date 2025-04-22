pub mod sorting;

use crate::sorting::bubble_sort::bubble_sort;
use crate::sorting::selection_sort::selection_sort;
use crate::sorting::display_arr::display_arr;

fn main() {
    let mut arr = vec![14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    bubble_sort(&mut arr);
    display_arr(arr);
    println!("===================");
    let mut vector = vec![14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    selection_sort(&mut vector);
    display_arr(vector);
}
