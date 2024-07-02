fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = ((low + high) / 2) as usize;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid as i32 + 1;
        } else {
            high = mid as i32 - 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 6;

    match binary_search(&arr, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }
}
