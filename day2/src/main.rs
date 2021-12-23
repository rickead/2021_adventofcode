// Day 2: 2021 Advent of Code
// Calculate the horizontal position and depth you would have after following the planned course.
// What do you get if you multiply your final horizontal position by your final depth?

// To solve this problem, we must read the data input, parse it and create a list of 
// x,y vectors where each vector is the positional adjustments for the sub.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;

use regex::Regex;

struct Direction {
    hor_position: i32,
    depth: i32,
}

fn main() {
    println!("Calculate the horizontal position and depth you would have after following the planned course.");
    println!("What do you get if you multiply your final horizontal position by your final depth? ");

    let sub_directions = read_input("./data.txt");
    let final_position = propel_sub(sub_directions);
    println!("     Horizontal position {}, depth {}, area {}", final_position.hor_position, final_position.depth, final_position.hor_position * final_position.depth);
}

fn parse_direction(text: &str) -> Direction {
    lazy_static! {
        static ref FORWARD_REGEX : Regex = Regex::new(
                r"forward\s(\d*)"
            ).unwrap();
        static ref UP_REGEX : Regex = Regex::new(
                r"up\s(\d*)"
            ).unwrap();
        static ref DOWN_REGEX : Regex = Regex::new(
                r"down\s(\d*)"
            ).unwrap();
    }
    let fwd_capture = FORWARD_REGEX.captures(text);
    let up_capture = UP_REGEX.captures(text);
    let down_capture = DOWN_REGEX.captures(text);

    if let Some(match_value) = fwd_capture {
        let value: i32 = match_value.get(1).unwrap().as_str().parse().unwrap();
        return Direction{ hor_position: value, depth: 0};      
    }
    else if let Some(match_value) = up_capture {
        let value: i32 = match_value.get(1).unwrap().as_str().parse().unwrap();
        return Direction{ hor_position: 0, depth: -value};
    }
    else if let Some(match_value) = down_capture {
        let value: i32 = match_value.get(1).unwrap().as_str().parse().unwrap();
        return Direction{ hor_position: 0, depth: value};      
    }
    else {
        return Direction{hor_position: 0, depth: 0};
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input(input_file: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                directions.push(parse_direction(&ip));
            }
        }
    }
    return directions;
}

fn propel_sub(directions: Vec<Direction>) -> Direction {

    let mut final_position = Direction{ hor_position:0, depth:0};

    for elem in directions {
        final_position.hor_position += elem.hor_position;
        final_position.depth += elem.depth;
    }

    return final_position;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_direction() {

        let mut d = parse_direction("forward 5");
        assert_eq!(d.hor_position, 5);
        assert_eq!(d.depth, 0);

        d = parse_direction("down 42");
        assert_eq!(d.hor_position, 0);
        assert_eq!(d.depth, 42);

        d = parse_direction("up 7");
        assert_eq!(d.hor_position, 0);
        assert_eq!(d.depth, -7);
    }

    #[test]
    fn test_read_input() {
        let directions = read_input("./test.txt");
        assert_eq!(directions.len(), 6);
        assert_eq!(directions[0].hor_position, 5);
        assert_eq!(directions[0].depth, 0);
        assert_eq!(directions[1].hor_position, 0);
        assert_eq!(directions[1].depth, 5);
        assert_eq!(directions[2].hor_position, 8);
        assert_eq!(directions[2].depth, 0);
        assert_eq!(directions[3].hor_position, 0);
        assert_eq!(directions[3].depth, -3);
        assert_eq!(directions[4].hor_position, 0);
        assert_eq!(directions[4].depth, 8);
        assert_eq!(directions[5].hor_position, 2);
        assert_eq!(directions[5].depth, 0);
    }

    #[test]
    fn test_propel_sub() {

        let directions = vec![ 
            Direction{hor_position:5,depth:0},
            Direction{hor_position:0,depth:5},
            Direction{hor_position:8,depth:0},
            Direction{hor_position:0,depth:-3},
            Direction{hor_position:0,depth:8},
            Direction{hor_position:2,depth:0},

        ];

        let final_position = propel_sub(directions);
        assert_eq!(final_position.hor_position, 15);
        assert_eq!(final_position.depth, 10);
    }
}