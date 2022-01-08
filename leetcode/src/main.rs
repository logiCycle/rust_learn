use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let mut nums1 = vec![1, 0, 1];
    move_zeroes(&mut nums1);
    println!("{:?}", nums1);
}
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let row_len = matrix.len();
    let col_len = matrix[0].len();
    for i in 0..(row_len / 2) {
        for j in i..col_len - 1 - i {
            let tmp = matrix[i][j];
            //(i,j)->(j,row - 1 - i)->(row - 1 - i, col - 1 - j)->(col - 1 - j, i)[i=row - 1 - (row - 1 - i)]
            matrix[i][j] = matrix[col_len - 1 - j][i];
            matrix[col_len - 1 - j][i] = matrix[row_len - 1 - i][col_len - 1 - j];
            matrix[row_len - 1 - i][col_len - i - j] = matrix[j][row_len - 1 - i];
            matrix[j][row_len - 1 - i] = tmp;
        }
    }
}

pub fn rotate_2(matrix: &mut Vec<Vec<i32>>) {
    let matrix_len = matrix.len();
    for i in 0..(matrix_len / 2) {
        matrix.swap(i, matrix_len - 1 - i);
    }
    for i in 0..matrix_len {
        for j in i + 1..matrix_len {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        return check_row(&board) && check_col(&board) && check_3x3(&board);
        fn check_row(board: &Vec<Vec<char>>) -> bool {
            for row_vec in board {
                let mut set = HashSet::with_capacity(9);
                for row_element in row_vec {
                    if *row_element == '.' {
                        continue;
                    }
                    if set.insert(row_element) == false {
                        return false;
                    }
                }
            }
            true
        }
        fn check_col(board: &Vec<Vec<char>>) -> bool {
            for col_index in 0..board.len() {
                let mut set = HashSet::with_capacity(9);
                for row_index in 0..board.len() {
                    if board[row_index][col_index] == '.' {
                        continue;
                    }
                    if !set.insert(&board[row_index][col_index]) {
                        return false;
                    }
                }
            }
            true
        }
        fn check_3x3(board: &Vec<Vec<char>>) -> bool {
            let mut set = HashSet::with_capacity(9);
            for times in 0..3 {
                let row_start = times * 3;
                for col_index in 0..board.len() {
                    if col_index % 3 == 0 {
                        set = HashSet::with_capacity(9);
                    }
                    for row_index in row_start..row_start + 3 {
                        if board[row_index][col_index] == '.' {
                            continue;
                        }
                        if !set.insert(&board[row_index][col_index]) {
                            return false;
                        }
                    }
                }
            }
            true
        }
    }
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut plus = 1;
    let mut res = digits;
    for element in res.iter_mut().rev() {
        *element = match (*element + plus) % 10 {
            0 => 0,
            add_value => {
                plus = 0;
                add_value
            }
        };
    }
    if plus == 1 {
        res.insert(0, 1);
    }
    res
}

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hash1 = HashMap::new();
    let mut res: Vec<i32> = vec![];
    for i in &nums1 {
        let v = hash1.entry(i).or_insert(0);
        *v += 1;
    }

    for i in &nums2 {
        if let Some(v) = hash1.get_mut(i) {
            if *v > 0 {
                res.push(*i);
                *v -= 1;
            }
        }
    }
    res
}

/*
pub fn rotate(nums: &mut Vec<i32>, k: i32){
    let len = nums.len() as i32;
    let start = (len - k % len) as usize;
    let mut tmp = Vec::new();
    tmp.extend_from_slice(&nums[start..]);
    tmp.extend_from_slice(&nums[..start]);
    nums.clear();
    nums.append(&mut tmp);
}
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    for _ in 0..k {
        let last_num = nums[len - 1];
        for j in (1..len).rev() {
            nums[j] = nums[j - 1];
        }
        nums[0] = last_num;
    }
}
*/
