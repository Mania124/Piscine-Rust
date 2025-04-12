pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut left: usize = 0;
    let mut right: usize = array.len();

    while left <= right {
        let middle: usize = (left + right) / 2;

        let element = array.get(middle)?; // ? is used to fast error handling
        if key < *element {
            right = middle.checked_sub(1)?; // sub two num, checks underflow. If underflow, (None) is returned.
        } else if key > *element {
            left = middle + 1;
        } else {
            return Some(middle);
        }
    }

    return None;
}