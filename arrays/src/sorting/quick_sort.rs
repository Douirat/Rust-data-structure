pub fn quick_sort(arr: &mut [isize; 12], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mut pivot_index = start;
    let pivot = arr[end];
    for i in start..end {
        if arr[i] < pivot {
            let temp = arr[i];
            arr[i] = arr[pivot_index];
            arr[pivot_index] = temp;
            pivot_index += 1;
        }
    }
    let temp = arr[end];
    arr[end] = arr[pivot_index];
    arr[pivot_index] = temp;

    if pivot_index > 0 {
        quick_sort(arr, start, pivot_index - 1);
    }
    quick_sort(arr, pivot_index + 1, end);
}
