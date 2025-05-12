// use crate::display_vector;

pub fn count_sort(arr: &mut Vec<isize>) {
    let mut max = arr[0];
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    // println!("{}", max);
    let mut count_vec = vec![0; (max+1) as usize];
    // display_vector(&count_vec);
    for i in 0..arr.len()-1 {
        count_vec[arr[i] as usize] += 1;
    }
    display_vector(&count_vec);
}

pub fn display_vector(vector: &Vec<isize>) {
    for val in vector {
        println!("{}", val);
    }
}
