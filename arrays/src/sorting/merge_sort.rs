pub fn merge_sort(arr: &mut[isize; 16], left: usize, right: usize){
if left < right {
    let middle = left + (right - left) / 2;
    println!("--> {}", middle);
    merge_sort(arr, left, middle);
    merge_sort(arr, middle+1, right);
}
}

fn merge(arr: &[isize; 16], l, m, r){

}