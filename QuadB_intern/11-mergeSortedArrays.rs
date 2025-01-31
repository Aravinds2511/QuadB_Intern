//11. Merge two sorted arrays in Rust
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    //remaining elements
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    return merged;
}

fn main() {
    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged Array: {:?}", merged_array);
}
