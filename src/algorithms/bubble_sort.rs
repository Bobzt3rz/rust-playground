pub fn bubble_sort(array: &mut [i32]) -> &mut [i32]{
    let end = array.len() - 1;
        for i in 0..end {
            // the last index will be sorted as highest number after each loop
            for j in 0..end - i {
                if array[j] > array[j+1]{
                    let tmp = array[j];
                    array[j] = array[j+1];
                    array[j+1] = tmp;
                }
            }
        }


    return array;
}

pub fn bubble_sort_test(){
    let mut insert_array:[i32;7] = [9, 3, 7, 4, 69, 420, 42];
    let out_array = bubble_sort(&mut insert_array);
    const EXPECTED_ARRAY: [i32;7] = [3, 4, 7, 9, 42, 69, 420];
    assert_eq!(out_array, EXPECTED_ARRAY);
}