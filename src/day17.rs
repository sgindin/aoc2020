use crate::tools;

const WIDTH: i64 = 20;
const DEPTH: i64 = 20;
const HEIGHT: i64 = 13;
const WEIRD: i64 = 13;

fn count_active_neighbor_3d(grid: &Vec<Vec<Vec<char>>>, x: i64, y: i64, z: i64) -> u32 {
    if x > 0 && x < WIDTH && y > 0 && y < DEPTH && z > 0 && z < HEIGHT {
        match grid[x as usize][y as usize][z as usize] {
            '#' => return 1,
            '.' => return 0,
            _ => unreachable!(),
        }
    }
    0
}

fn count_active_neighbor_4d(grid: &Vec<Vec<Vec<Vec<char>>>>, x: i64, y: i64, z: i64, w: i64) -> u32 {
    if x > 0 && x < WIDTH && y > 0 && y < DEPTH && z > 0 && z < HEIGHT && w > 0 && w < WEIRD {
        match grid[x as usize][y as usize][z as usize][w as usize] {
            '#' => return 1,
            '.' => return 0,
            _ => unreachable!(),
        }
    }
    0
}

lazy_static! {
    static ref NEIGHBORS_3D: Vec<Vec<i64>> = {
        let mut cs = vec![];
        for x in -1..=1i64 {
            for y in -1..=1i64 {
                for z in -1..=1i64 {
                    if !(x == 0 && y == 0 && z == 0) {
                        cs.push(vec![x, y, z]);
                    }
                }
            }
        }
        cs
    };
    static ref NEIGHBORS_4D: Vec<Vec<i64>> = {
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

fn count_active_neighbors_3d(grid: &Vec<Vec<Vec<char>>>, x: usize, y: usize, z: usize) -> u32 {
    NEIGHBORS_3D.iter().fold(0u32, |acc, c|
        acc + count_active_neighbor_3d(&grid, x as i64 + c[0], y as i64 + c[1], z as i64 + c[2])
    )
}

fn count_active_neighbors_4d(grid: &Vec<Vec<Vec<Vec<char>>>>,
                             x: usize, y: usize, z: usize, w: usize) -> u32 {
    NEIGHBORS_4D.iter().fold(0u32, |acc, c|
        acc + count_active_neighbor_4d(&grid, x as i64 + c[0], y as i64 + c[1], z as i64 + c[2], w as i64 + c[3])
    )
}

pub fn solve() {
    let mut grid_3d =
        vec![vec![vec!['.'; HEIGHT as usize]; DEPTH as usize]; WIDTH as usize];
    let mut grid_4d =
        vec![vec![vec![vec!['.'; WEIRD as usize]; HEIGHT as usize]; DEPTH as usize]; WIDTH as usize];
    tools::read_lines("./input/day17.txt")
        .unwrap()
        .enumerate()
        .for_each(|(y, line)| {
            line.unwrap().chars().enumerate()
                .for_each(|(x, c)| {
                    grid_3d[x + 6][y + 6][6] = c;
                    grid_4d[x + 6][y + 6][6][6] = c;
                });
        });

    let mut next_grid_3d = grid_3d.clone();
    let mut active_cubes_3d = 0;
    let mut next_grid_4d = grid_4d.clone();
    let mut active_cubes_4d = 0;
    for _ in 0..6 {
        active_cubes_3d = 0;
        active_cubes_4d = 0;
        for x in 0..WIDTH as usize {
            for y in 0..DEPTH as usize {
                for z in 0..HEIGHT as usize {
                    next_grid_3d[x][y][z] = grid_3d[x][y][z];
                    let active_neighbors = count_active_neighbors_3d(&grid_3d, x, y, z);
                    match grid_3d[x][y][z] {
                        '#' => {
                            if !(active_neighbors == 2 || active_neighbors == 3) {
                                next_grid_3d[x][y][z] = '.';
                            } else {
                                active_cubes_3d += 1;
                            }
                        },
                        '.' => if active_neighbors == 3 {
                            next_grid_3d[x][y][z] = '#';
                            active_cubes_3d += 1;
                        },
                        _ => unreachable!()
                    }
                    for w in 0..WEIRD as usize {
                        next_grid_4d[x][y][z][w] = grid_4d[x][y][z][w];
                        let active_neighbors = count_active_neighbors_4d(&grid_4d, x, y, z, w);
                        match grid_4d[x][y][z][w] {
                            '#' => {
                                if !(active_neighbors == 2 || active_neighbors == 3) {
                                    next_grid_4d[x][y][z][w] = '.';
                                } else {
                                    active_cubes_4d += 1;
                                }
                            },
                            '.' => if active_neighbors == 3 {
                                next_grid_4d[x][y][z][w] = '#';
                                active_cubes_4d += 1;
                            },
                            _ => unreachable!()
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut grid_3d, &mut next_grid_3d);
        std::mem::swap(&mut grid_4d, &mut next_grid_4d);
    }
    println!("{}, {}", active_cubes_3d, active_cubes_4d)
}