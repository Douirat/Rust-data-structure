pub fn radix_sort(arr: &mut[isize; 12]){
    let max = get_max(arr);
    println!("-----> {}", max);
    let mut place = 1;
    while (max / place) > 0 {
        count_sort(arr, place);
        place *= 10;
    }
}

fn count_sort(arr: &mut[isize; 12], place: isize) {
      println!("-----> {}", place);
      let mut count_arr = [0; 10];
      let mut output = [0; 12];

      for i in 0..arr.len(){
        let ind = (arr[i] / place) % 10;
        count_arr[ind as usize] += 1;
      }
      for x in 1..count_arr.len(){
        count_arr[x] += count_arr[x-1]
      }

      for z in (0..arr.len()).rev() {
        let index = (arr[z] / place) % 10;
        output[count_arr[index as usize] as usize -1] = arr[z];
        count_arr[index as usize] -= 1;
       }
       for f in 0..arr.len(){
        arr[f] = output[f];
       }
}

fn get_max(arr: &[isize; 12]) -> isize {
    let mut max = arr[0];
    for  i in 1..arr.len() {
        if max < arr[i] {
            max = arr[i];
        }
    }
    max
}