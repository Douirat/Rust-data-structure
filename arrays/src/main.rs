pub mod sorting;

use crate::sorting::bubble_sort::bubble_sort;
use crate::sorting::selection_sort::selection_sort;
use crate::sorting::insertion_sort::insertion_sort;
use crate::sorting::display_arr::display_arr;
use crate::sorting::display_vector::display_vector;

fn main() {
    println!("\n========> bubble sort! <===========\n");
    let mut arr = vec![14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    bubble_sort(&mut arr);
    display_vector(arr);
    println!("\n=========> selection sort! <==========\n");
    let mut array = [14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    selection_sort(&mut array);
    display_arr(array);
    println!("\n=========> insertion sort <==========\n");
    let mut array1 = [14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    insertion_sort(&mut array1);
    display_arr(array1);
}
