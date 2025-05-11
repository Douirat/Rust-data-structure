pub fn selection_sort(arr: &mut [isize; 12]) {
    for i in 0..arr.len()-1{
        let mut swapped = false;
        let mut min = i;
        for j  in i+1..arr.len(){
            if arr[j] <= arr[min] {
                min = j;
                swapped = true;
            }
        }
        if swapped{
            let temp = arr[i];
            arr[i] = arr[min];
            arr[min] = temp;
        }
    }
}