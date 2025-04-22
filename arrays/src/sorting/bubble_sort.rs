
pub fn bubble_sort(arr: &mut Vec<isize>) {
    for i in 0..arr.len()-1{
        for j in i+1..arr.len(){
                if arr[i] >= arr[j]{
                    arr[i] = arr[i] + arr[j];
                    arr[j] = arr[i] - arr[j];
                    arr[i] = arr[i] - arr[j];
                }
        }
    }
}