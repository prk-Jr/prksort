pub fn sort<T: PartialEq + PartialOrd + Copy>(arr: &mut Vec<T>) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    let mut left = arr[..mid].to_vec();

    let mut right = arr[mid..].to_vec();

    sort(&mut left);

    sort(&mut right);

    merge(&mut left, &mut right, arr);
}

fn merge<T: PartialOrd + Copy + PartialEq>(left: &mut [T], right: &mut [T], result: &mut [T]) {
    let mut left_idx = 0;

    let mut right_idx = 0;

    let mut result_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            result[result_idx] = left[left_idx];

            left_idx += 1;
        } else {
            result[result_idx] = right[right_idx];

            right_idx += 1;
        }

        result_idx += 1;
    }

    while left_idx < left.len() {
        result[result_idx] = left[left_idx];

        left_idx += 1;

        result_idx += 1;
    }

    while right_idx < right.len() {
        result[result_idx] = right[right_idx];

        right_idx += 1;

        result_idx += 1;
    }
}
