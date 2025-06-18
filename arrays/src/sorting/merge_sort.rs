pub fn merge_sort(vec: Vec<isize>)  -> Vec<isize> {
if vec.len() <= 1 {
return vec;
}
let mid = vec.len() / 2;
let left = merge_sort(vec[..mid].to_vec());
let right = merge_sort(vec[mid..].to_vec());
return merge(left, right);
}

fn merge(left: Vec<isize>, right: Vec<isize>) -> Vec<isize> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i+= 1;
        } else {
            merged.push(right[j]);
            j+=1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i+=1;
    }

    while j < right.len() {
       merged.push(right[j]);
            j+=1;
    }

 return merged
}