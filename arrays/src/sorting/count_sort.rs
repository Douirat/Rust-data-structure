// use crate::display_vector;

pub fn count_sort(arr: &mut Vec<isize>) {
    let mut max = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    // println!("{}", max);
    let mut  count_vec = vec![0; (max+1) as usize];
    // display_vector(&count_vec);

    
    for i in 0.. arr.len() {
        count_vec[arr[i]as usize ] += 1;
    };
    
    for i in 1..count_vec.len(){
        count_vec[i] += count_vec[i -1]
    }
    let mut output = vec![0; arr.len() as usize];
    //     output[count_vec[arr[i] as usize]as usize] = arr[i];
    for i in (0..output.len()).rev(){
        let index = count_vec[arr[i] as usize] as usize -1 ;
       output[index] = arr[i];
        count_vec[arr[i] as usize] -= 1;
    }
    for i in 0.. arr.len(){
        arr[i] = output[i];
    }

 
}
