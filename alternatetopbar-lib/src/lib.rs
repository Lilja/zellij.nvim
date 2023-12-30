fn find_middle(cols: usize) -> usize {
    if cols % 2 != 0 {
        return (cols / 2) + 1;
    }
    return cols / 2;
}

fn certain_side_priority(cols: usize, side_size: usize, center_size: usize, padding: usize) -> usize {
    // good luck understanding this code
    let center_div = center_size / 2;
    if cols % 2 == 0 {
        if (cols / 2) - side_size < center_div {
            0
        } else {
            (cols / 2) - side_size - center_div
        }
    } else {
        if (cols / 2) - side_size < center_div {
            0
        } else {
            let calc = (cols / 2) - side_size - center_div;
            if calc > padding {
                0
            } else {
                calc
            }
        }
    }
}
pub fn find_padding(
    cols: usize,
    left_size: usize,
    center_size: usize,
    right_size: usize,
) -> (usize, usize) {
    let total_size = left_size + center_size + right_size;
    if total_size >= cols {
        return (0, 0);
    }
    let padding = cols - total_size;
    let middle = find_middle(cols);

    let is_left_side = if left_size >= middle {
        true
    } else if left_size > right_size {
        true
    } else {
        false
    };
    let padding_side = if left_size >= middle {
        0
    } else {
        // eprintln!("which side? {}", if is_left_side { "left" } else { "right" });
        certain_side_priority(
            cols,
            if is_left_side { left_size } else { right_size },
            center_size,
            padding,
        )
    };

    let padding_left;
    let padding_right;

    if is_left_side {
        padding_left = if left_size >= middle { 0 } else { padding_side };
        padding_right = padding - padding_left;
    } else {
        padding_right = padding_side;
        padding_left = padding - padding_right;
    }

    (padding_left, padding_right)
}

// TODO: Figure out if just placing the left/right parts into the string is a better way...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let result = find_padding(10, 2, 2, 2);
        /*
         * [LL**CC**RR]
         */
        assert_eq!(result, (2, 2));
    }

    #[test]
    fn two() {
        let result = find_padding(14, 2, 2, 2);
        /*           11111
         *  12345678901234
         * [LL****CC****RR]
         */
        assert_eq!(result, (4, 4));
    }

    #[test]
    fn three() {
        let result = find_padding(14, 3, 2, 3);
        /*           11111
         *  12345678901234
         * [LLL***CC***RRR]
         */
        assert_eq!(result, (3, 3));
    }

    #[test]
    fn left_bigger() {
        let result = find_padding(14, 6, 2, 3);
        /*           11111
         *  12345678901234
         * [LLLLLLCC***RRR]
         */
        assert_eq!(result, (0, 3));
        let result2 = find_padding(14, 7, 2, 3);
        /*           11111
         *  12345678901234
         * [LLLLLLLCC**RRR]
         */
        assert_eq!(result2, (0, 2));
        let result3 = find_padding(14, 5, 3, 3);
        /*           11111
         *  12345678901234
         * [LLLLL*CCC**RRR]
         */
        assert_eq!(result3, (1, 2));
        let result4 = find_padding(14, 6, 1, 1);
        /*           11111
         *  12345678901234
         * [LLLLLL*C*****R]
         * 14-(5+1+1)=14-8=6 padding
         * (14/2)-6=7-6=1
         */
        assert_eq!(result4, (1, 5));
    }

    #[test]
    fn middle_bigger() {
        let result = find_padding(14, 1, 7, 1);
        /*           11111
         *  12345678901234
         * [L***CCCCCCC**R]
         */
        assert_eq!(result, (2, 3));
    }

    #[test]
    fn right_bigger() {
        let result = find_padding(14, 1, 1, 6);

        /*           11111
         *  12345678901234
         * [L*****C*RRRRRR]
         */
        assert_eq!(result, (5, 1));
    }

    #[test]
    fn uneven() {
        let result = find_padding(7, 2, 2, 2);
        /*
         * [LLCC*RR]
         */
        assert_eq!(result, (1, 0));
    }

    #[test]
    fn crash() {
        let result = find_padding(105, 12, 82, 6);
        /*
         * [LLLLLLLLLLLLCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC*****RRRRRR]
         */
        assert_eq!(result, (0, 5));
    }

    #[test]
    fn imported_python_test_cases() {
        assert_eq!(find_padding(11, 2, 2, 2), (3, 2));
        assert_eq!(find_padding(11, 3, 1, 1), (2, 4));
        assert_eq!(find_padding(11, 3, 1, 3), (2, 2));
        assert_eq!(find_padding(6, 3, 1, 1), (0, 1));
        assert_eq!(find_padding(6, 3, 1, 1), (0, 1));
        assert_eq!(find_padding(11, 3, 2, 2), (1, 3));
        //           11
        //  12345678901
        // [LLL*CC***RR]
        assert_eq!(find_padding(14, 2, 2, 2), (4, 4));
        assert_eq!(find_padding(10, 2, 2, 2), (2, 2));
        assert_eq!(find_padding(14, 5, 3, 3), (1, 2));
        assert_eq!(find_padding(14, 7, 2, 3), (0, 2));
        assert_eq!(find_padding(7, 2, 2, 2), (1, 0));
    }

    #[test]
    fn wtf() {
        assert_eq!(find_padding(255, 18, 63, 9), (78, 87));
    }
}
