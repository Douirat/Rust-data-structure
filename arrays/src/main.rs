pub mod sorting;

use crate::sorting::bubble_sort::bubble_sort;
use crate::sorting::selection_sort::selection_sort;
use crate::sorting::insertion_sort::insertion_sort;
use crate::sorting::display_arr::display_arr;
use crate::sorting::display_vector::display_vector;
use crate::sorting::quick_sort::quick_sort;
use crate::sorting::count_sort::count_sort;

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
    println!("\n=========> Quick sort <==========\n");
    let mut array2 = [14, 5, 26, 6, 98, 7, 45, 2, 14, 5, 87, 56];
    quick_sort(&mut array2, 0, 11);
    display_arr(array2);
    println!("\n=========> count sort <==========\n");
    let mut vec_arr = vec![4, 2, 4, 2, 1, 5, 2, 1, 5, 2];
    count_sort(&mut vec_arr);
    display_vector(vec_arr);
    println!("\n=========> radix sort <==========\n");
    let  arr3 =  [170, 45, 75, 90, 322, 74, 333, 534, 802, 24, 2, 66];
    display_arr(arr3);
}
