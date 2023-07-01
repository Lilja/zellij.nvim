
pub fn calculate_left_padding(middle: usize, left_size: usize, center_size: usize) -> usize {
    if (middle - left_size) < center_size / 2 {
        return 0;
    }
    return middle - left_size - center_size / 2;
}

pub fn calculate_right_padding(
    cols: usize,
    middle: usize,
    right_size: usize,
    center_size: usize,
    left_padding_zero_compensation: usize,
    free_space: usize,
) -> usize {

    /*
    eprintln!(
        "Right padding: middle: {} right_size: {} center_size: {}, product: {}",
        middle,
        right_size,
        center_size,
        (middle - right_size - center_size)
    );
    */
    if left_padding_zero_compensation == 0 {
        return free_space;
    }
    if (middle - right_size) < center_size {
        return 0;
    }
    let mut extra_spacing: isize = 0;
    let mut calc = (middle - right_size - center_size);
    if cols % 2 != 0 {
        // If even number of cols, add + to spacing if not full
        extra_spacing = -1;
        calc = (middle - right_size - center_size)-1;
    }
    eprintln!("calc: {}, extra?: {}", calc, extra_spacing);
    // Because it's offshot by 1.
    if calc > 0 {
        return calc;
    }
    return calc;
}

pub fn find_middle(cols: usize) -> usize {
    if cols % 2 != 0 {
        return (cols/2)+1;
    }
    return cols/2;
}
