use crate::tools;

fn count_active_neighbor(grid: &Vec<Vec<Vec<Vec<char>>>>,
                         mut x: i64, mut y: i64, mut z: i64, mut w: i64,
                         x_inc: i64, y_inc: i64, z_inc: i64, w_inc: i64) -> u32 {
    let width = grid.len() as i64;
    let depth = grid[0].len() as i64;
    let height = grid[0][0].len() as i64;
    let weird = grid[0][0][0].len() as i64;
    x += x_inc; y += y_inc; z += z_inc; w += w_inc;
    if x > 0 && x < width && y > 0 && y < depth && z > 0 && z < height && w > 0 && w < weird {
        match grid[x as usize][y as usize][z as usize][w as usize] {
            '#' => return 1,
            '.' => return 0,
            _ => unreachable!(),
        }
    }
    0
}

lazy_static! {
    static ref NEIGHBORS: Vec<Vec<i64>> = {
        let mut cs = vec![];
        for x in -1..=1i64 {
            for y in -1..=1i64 {
                for z in -1..=1i64 {
                    for w in -1..=1i64 {
                        if !(x == 0 && y == 0 && z == 0 && w == 0) {
                            cs.push(vec![x, y, z, w]);
                        }
                    }
                }
            }
        }
        cs
    };
}

fn count_active_neighbors(grid: &Vec<Vec<Vec<Vec<char>>>>,
                          x: usize, y: usize, z: usize, w: usize) -> u32 {
    NEIGHBORS.iter().fold(0u32, |acc, c| {
        acc + count_active_neighbor(&grid, x as i64, y as i64, z as i64, w as i64,
                                    c[0], c[1], c[2], c[3])
    })
}

pub fn solve() {
    let mut grid = vec![vec![vec![vec!['.'; 13]; 13]; 20]; 20];
    tools::read_lines("./input/day17.txt")
        .unwrap()
        .enumerate()
        .for_each(|(y, line)| {
            line.unwrap().chars().enumerate()
                .for_each(|(x, c)| grid[x + 6][y + 6][6][6] = c);
        });

    let mut next_grid = grid.clone();
    let mut active_cubes = 0;
    for _ in 0..6 {
        active_cubes = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                for z in 0..grid[0][0].len() {
                    for w in 0..grid[0][0][0].len() {
                        next_grid[x][y][z][w] = grid[x][y][z][w];
                        let active_neighbors = count_active_neighbors(&grid, x, y, z, w);
                        match grid[x][y][z][w] {
                            '#' => {
                                if !(active_neighbors == 2 || active_neighbors == 3) {
                                    next_grid[x][y][z][w] = '.';
                                } else {
                                    active_cubes += 1;
                                }
                            },
                            '.' => if active_neighbors == 3 {
                                next_grid[x][y][z][w] = '#';
                                active_cubes += 1;
                            },
                            _ => unreachable!()
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next_grid);
    }
    println!("{}", active_cubes)
}