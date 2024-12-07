use std::cmp::min;
use crate::elves::read_grid;



fn get_slice_safe(row: &Vec<char>, start: usize, end: usize) -> Vec<char>{
    let slice_end = min(end, row.len());
    return row[start..slice_end].to_vec()
}

fn get_col_slice_safe(grid: &Vec<Vec<char>>, row: usize, col: usize, row_end: usize) -> Vec<char> {
    let slice_end = min(row_end, grid.len());
    let mut slice = Vec::new();
    for r in row..slice_end {
        slice.push(grid[r][col]);  
    }
    return slice;
}

fn get_diagonal_r_slice_safe(grid: &Vec<Vec<char>>, row: usize, col: usize, size: usize) -> Vec<char> {
    let row_end = min(row+size, grid.len());
    let col_end = min(col+size, grid[0].len());

    let mut slice = Vec::new();
    for (r, c) in (row..row_end).zip(col..col_end) {
        slice.push(grid[r][c]);  
    }
    return slice;
}

fn get_diagonal_l_slice_safe(grid: &Vec<Vec<char>>, row: usize, col: usize, size: usize) -> Vec<char> {
    let row_end = min(row+size, grid.len());
    println!("row {row} col {col}");
    if col+1 < size  {println!("Oh no!"); return Vec::new()};

    //println!("{}, {}, {}, {}", row, col, row_end, col-4);

    let mut slice = Vec::new();
    for (c_diff, r) in (row..row_end).enumerate() {
        slice.push(grid[r][col-c_diff]);  
    }
    if row == 2 {println!("{:?}", slice)};

    return slice;
}

pub fn part_1() {
    let grid: Vec<Vec<char>> = read_grid("input/day04");
    let xmas = ['X', 'M', 'A', 'S'];
    let samx = ['S', 'A', 'M', 'X'];

    let mut total = 0;
    for (row, row_address) in grid.iter().enumerate() {
        println!("{:?}", row);
        for (col, val) in row_address.iter().enumerate() {
            if *val == 'X' || *val == 'S' {
                let row_slice = get_slice_safe(row_address, col, col+4);
                let col_slice = get_col_slice_safe(&grid, row, col, row+4);
                let diag_r_slice = get_diagonal_r_slice_safe(&grid, row, col, 4);
                let diag_l_slice = get_diagonal_l_slice_safe(&grid, row, col, 4);

                if row == 2 {println!("val {} {col}", *val)};
                //also print if matches

                if row_slice == xmas || row_slice == samx {println!("r:{:?}", row_slice);total += 1};
                if col_slice == xmas || col_slice == samx {println!("c:{:?}", col_slice);total += 1};
                if diag_r_slice == xmas || diag_r_slice == samx {println!("dr:{:?} {} {}", diag_r_slice, row, col);total += 1};
                if diag_l_slice == xmas || diag_l_slice == samx {println!("dr:{:?} {} {}", diag_l_slice, row, col);total += 1};
            }
        }
    }
    println!("{}", total);
}



pub fn part_2() {
    let grid: Vec<Vec<char>> = read_grid("input/day04");
    let xmas = ['M', 'A', 'S'];
    let samx = ['S', 'A', 'M'];

    let mut total = 0;
    for (row, row_address) in grid.iter().enumerate() {
        println!("{:?}", row);
        for (col, val) in row_address.iter().enumerate() {
            if *val == 'M' || *val == 'S' {
                let diag_l_slice = get_diagonal_r_slice_safe(&grid, row, col, 3);
                if diag_l_slice == xmas || diag_l_slice == samx {
                    let diag_r_slice = get_diagonal_l_slice_safe(&grid, row, col+2, 3);
                    if diag_r_slice == xmas || diag_r_slice == samx {total += 1};
                };
            }
        }
    }
    println!("{}", total);
}