#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;

type Grid = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

impl Solution {
    pub fn num_islands(mut grid: Grid) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut result = 0;
        let rows = grid.len() - 1;
        let cols = grid[0].len() - 1;

        for i in 0..=rows {
            for j in 0..=cols {
                if grid[i][j] == '1' {
                    result += 1;

                    queue.push_back((i, j));

                    while let Some(front) = queue.pop_front() {
                        let (x, y) = front;

                        if grid[x][y] == '0' {
                            continue;
                        }

                        grid[x][y] = '0';

                        if x < rows {
                            queue.push_back((x + 1, y));
                        }

                        if x > 0 {
                            queue.push_back((x - 1, y));
                        }

                        if y < cols {
                            queue.push_back((x, y + 1));
                        }

                        if y > 0 {
                            queue.push_back((x, y - 1));
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, Solution};

    #[test]
    pub fn test_number_of_islands_case1() {
        let grid: Grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let expected = 1;

        let result = Solution::num_islands(grid);

        assert_eq!(
            result, expected,
            "Expected: {}, but Got: {}",
            expected, result
        );
    }
}
