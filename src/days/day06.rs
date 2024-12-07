use std::collections::HashSet;

use crate::elves::read_grid as read_grid;


pub fn part_1() {
    let grid: Vec<Vec<char>> = read_grid("input/day06");
    let map = Map::new(grid);
    let mut guard = Guard::new(map);

    while guard.state != State::Escaped {
        guard.step();
    }

    println!("Guard escaped after {} steps", guard.visited.len());
}

fn looped_exp(map: &Map, new_obs: Position) -> bool{
    let mut new_map = map.clone();
    new_map.insert_obstacle(new_obs);
    let mut guard = Guard::new(new_map);

    while guard.state == State::Searching {
        guard.step();
    }
    if guard.state == State::Looped {
        return true;
    }
    return false;
}

pub fn part_2() {
    let grid: Vec<Vec<char>> = read_grid("input/day06");
    let map = Map::new(grid);

    let mut total =0;
    for (i, row) in map.grid.iter().enumerate() {
        for (j, feature) in row.iter().enumerate() {
            match feature {
                Feature::Free => {
                    if map.start_position == (j as i32, i as i32) {
                        continue;
                    }
                    if looped_exp(&map, (j as i32, i as i32)) {
                        total += 1;     
                    }
                },
                Feature::Obstacle => continue,
            }
            println!("{}, {}", j, i);
        }
    }
    println!("{}", total);
}

type Position = (i32, i32);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn next_position(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1)
    }
}

fn rotate_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

struct Guard {
    facing: Direction,
    pos: Position,
    visited: HashSet<(Position, Direction)>,
    grid: Map,
    state: State
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Searching,
    Escaped,
    Looped
}

impl Guard {
    fn new(map: Map) -> Guard {

        Guard {
            facing: Direction::Up,
            pos: map.start_position,
            visited: HashSet::from([(map.start_position, Direction::Up)]),
            grid: map,
            state: State::Searching,
        }
    }

    fn step(&mut self) {
        let new_pos: Position = next_position(self.pos, self.facing);

        if new_pos.0 < 0 || new_pos.0 >= self.grid.width || new_pos.1 < 0 || new_pos.1 >= self.grid.height {
            self.state = State::Escaped;
            return;
        }
        let feature = self.grid.get(new_pos);
        match feature { 
            Feature::Free => {
                let new_visit = (new_pos, self.facing);
                self.pos = new_pos;
                if self.visited.contains(&new_visit) {
                    self.state = State::Looped;                    
                }
                self.visited.insert((new_pos, self.facing));

            },
            Feature::Obstacle => {
                self.facing = rotate_right(self.facing);
            }
        }
        
    }
}

#[derive(Copy, Clone, Debug)]
enum Feature {
    Obstacle,
    Free,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Feature>>,
    start_position: Position,
    width: i32,
    height: i32,
}

impl Map {
    fn new(char_grid: Vec<Vec<char>>) -> Map {
        let mut grid: Vec<Vec<Feature>> = Vec::new();
        let mut start_position: Position = (0, 0);
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for (y, row) in char_grid.iter().enumerate() {
            let mut grid_row = Vec::new();
            for (x, c) in row.iter().enumerate() {
                let feature = match c {
                    '.' => Feature::Free,
                    '#' => Feature::Obstacle,
                    '^' => {
                        println!("{}, {}", x, y);
                        start_position = (x as i32, y as i32);
                        Feature::Free
                    },
                    _ => panic!("Invalid character in grid")
                };
                grid_row.push(feature);
            }
            grid.push(grid_row);
        }

        width = grid[0].len() as i32;
        height = grid.len() as i32;

        Map {
            grid: grid,
            start_position: start_position,
            width: width,
            height: height,
        }
        
    }
    
    fn get(&self, pos: Position) -> Feature {
        self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn insert_obstacle(&mut self, pos: Position) {
        self.grid[pos.1 as usize][pos.0 as usize] = Feature::Obstacle;
    }
}
