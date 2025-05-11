pub fn insertion_sort(arr: &mut [isize; 12]){
for i in 1..arr.len(){
    let mut ind = i;
    while ind as isize > 0 && arr[ind] <= arr[ind-1]{
        let temp = arr[ind];
        arr[ind] = arr[ind-1];
        arr[ind-1] = temp;
        ind = ind -1;
    }
}
}