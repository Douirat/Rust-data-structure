pub fn selection_sort(arr: &mut Vec<isize>) {
    println!("{:?}", arr);
    for i in 0..arr.len(){
        let mut min = i;
        let mut swapped = false;
        for j in i+1..arr.len(){
            if arr[j] <= arr[min] {
                min = j;
                swapped = true;
            }
        }
        if swapped {
            arr[min] = arr[i] + arr[min];
            arr[i] = arr[min]  - arr[i];
            arr[min] = arr[min] - arr[i];
        }
    }
}
