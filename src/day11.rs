use crate::tools;

fn count_occupied_neighbors_in_direction(grid: &Vec<Vec<char>>, mut i: i64, mut j: i64,
                                         i_inc: i64, j_inc: i64, go_far: bool) -> u32 {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    i += i_inc;
    j += j_inc;
    while i >= 0 && i < height && j >= 0 && j < width {
        match grid[i as usize][j as usize] {
            '#' => return 1,
            'L' => return 0,
            '.' => (),
            _ => unreachable!(),
        }
        if !go_far {
            break;
        }
        i += i_inc;
        j += j_inc;
    }
    0
}

fn count_occupied_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize, go_far: bool) -> u32 {
    let i = i as i64;
    let j = j as i64;
      count_occupied_neighbors_in_direction(grid, i, j, -1, 0, go_far)  // up
    + count_occupied_neighbors_in_direction(grid, i, j, -1, -1, go_far) // up-left
    + count_occupied_neighbors_in_direction(grid, i, j, -1, 1, go_far)  // up-right
    + count_occupied_neighbors_in_direction(grid, i, j, 1, 0, go_far)   // down
    + count_occupied_neighbors_in_direction(grid, i, j, 1, -1, go_far)  // down-left
    + count_occupied_neighbors_in_direction(grid, i, j, 1, 1, go_far)   // down-right
    + count_occupied_neighbors_in_direction(grid, i, j, 0, -1, go_far)  // left
    + count_occupied_neighbors_in_direction(grid, i, j, 0, 1, go_far)   // right
}

fn count_stable_occupied_neighbors(grid: &Vec<Vec<char>>, go_far: bool, occupation_limit: u32) -> u32 {
    let mut grid = grid.clone();
    let height = grid.len();
    let width = grid[0].len();
    let mut next_grid = grid.clone();
    let mut occupation_is_stable = false;
    let mut occupied_neighbors_count = 0;
    while !occupation_is_stable {
        occupation_is_stable = true;
        occupied_neighbors_count = 0;
        for i in 0..height {
            for j in 0..width {
                next_grid[i][j] = grid[i][j];
                match grid[i][j] {
                    'L' => if count_occupied_neighbors(&grid, i, j, go_far) == 0 {
                        next_grid[i][j] = '#';
                        occupation_is_stable = false;
                    },
                    '#' => if count_occupied_neighbors(&grid, i, j, go_far) >= occupation_limit {
                        next_grid[i][j] = 'L';
                        occupation_is_stable = false;
                    } else {
                        occupied_neighbors_count += 1;
                    },
                    '.' => (),
                    _ => unreachable!(),
                }
            }
        }
        std::mem::swap(&mut grid, &mut next_grid);
    }
    occupied_neighbors_count
}

pub fn solve() {
    let grid = tools::read_lines("./input/day11.txt")
        .unwrap()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    assert_ne!(height, 0);
    let width = grid[0].len();
    assert!(grid.iter().all(|l| l.len() == width));

    let part1 = count_stable_occupied_neighbors(&grid, false, 4);
    let part2 = count_stable_occupied_neighbors(&grid, true, 5);

    print!("{}, {}", part1, part2)
}
