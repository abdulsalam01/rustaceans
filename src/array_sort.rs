
pub fn sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}