//
// Maximum path sum I, v1
// https://projecteuler.net/problem=18
//
// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
// the maximum total from top to bottom is 23.
//        3
//       7 4
//      2 4 6
//     8 5 9 3
// That is, 3 + 7 + 4 + 9 = 23.
// Find the maximum total from top to bottom of the triangle below:
// NOTE: triangle put in code
//

const SIZE: usize = 15;

fn triangle_path_sum(triangle: &[[u64; SIZE]; SIZE], (row, col): (usize, usize)) -> u64 {
    if row == SIZE - 1 {
        return triangle[row][col];
    }
    let left = triangle[row][col] + triangle_path_sum(triangle, (row + 1, col));
    let right = triangle[row][col] + triangle_path_sum(triangle, (row + 1, col + 1));
    return if left >= right { left } else { right };
}

pub fn run() {
    let triangle: [[u64; SIZE]; SIZE] = [
        [75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [95, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [17, 47, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [18, 35, 87, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [20, 4, 82, 47, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [19, 1, 23, 75, 3, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [88, 2, 77, 73, 7, 63, 67, 0, 0, 0, 0, 0, 0, 0, 0],
        [99, 65, 4, 28, 6, 16, 70, 92, 0, 0, 0, 0, 0, 0, 0],
        [41, 41, 26, 56, 83, 40, 80, 70, 33, 0, 0, 0, 0, 0, 0],
        [41, 48, 72, 33, 47, 32, 37, 16, 94, 29, 0, 0, 0, 0, 0],
        [53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 0, 0, 0, 0],
        [70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, 0, 0, 0],
        [91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 0, 0],
        [63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 0],
        [4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ];
    println!("Result: {}", triangle_path_sum(&triangle, (0, 0)));
}
