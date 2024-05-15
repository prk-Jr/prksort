pub fn sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_pioint = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_pioint);

    sort(&mut left[..pivot_pioint]);

    sort(&mut right[1..])
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
    let mut i = 0;

    let pivot_pos = arr.len() - 1;

    let pivot = arr[pivot_pos];

    for j in 0..pivot_pos {
        if arr[j] <= pivot {
            arr.swap(i, j);

            i += 1;
        }
    }

    arr.swap(i, pivot_pos);

    i
}
