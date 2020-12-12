use crate::tools;
use std::collections::HashMap;

#[derive(Default)]
struct Bearings {
    lon: i32,
    lat: i32,
    heading: i32,
}

lazy_static! {
    static ref ANGLE_AS_CHAR: HashMap<i32, char> =
        [(0, 'E'), (90, 'N'), (180, 'W'), (270, 'S')].iter().cloned().collect();
}

impl Bearings {
    pub fn new(lon: i32, lat: i32) -> Bearings {
        Bearings { lon, lat, heading: 0 }
    }

    pub fn turn(&mut self, direction: char, angle: i32) {
        let sign = match direction {
            'L' => 1,
            'R' => -1,
            _ => unreachable!(),
        };
        self.heading = (360 + self.heading + sign * angle).abs() % 360;
    }

    pub fn rotate(&mut self, direction: char, angle: i32) {
        let angle = match direction {
            'L' => 360 - angle,
            'R' => angle,
            _ => unreachable!(),
        };
        match angle {
            90 => {
                std::mem::swap(&mut self.lat, &mut self.lon);
                self.lat = -self.lat;
            },
            180 => {
                self.lon = -self.lon;
                self.lat = -self.lat;
            },
            270 => {
                std::mem::swap(&mut self.lat, &mut self.lon);
                self.lon = -self.lon;
            },
            _ => unreachable!()
        };
    }

    pub fn shift_in_direction(&mut self, heading: char, distance: i32) {
        match heading {
            'E' => self.lon += distance,
            'N' => self.lat += distance,
            'W' => self.lon -= distance,
            'S' => self.lat -= distance,
            _ => unreachable!(),
        };
    }

    pub fn shift(&mut self, distance: i32) {
        self.shift_in_direction(ANGLE_AS_CHAR[&self.heading], distance);
    }

    pub fn shift_to_waypoint(&mut self, waypoint: &Bearings, repeat: i32) {
        self.lon += repeat * waypoint.lon;
        self.lat += repeat * waypoint.lat;
    }

    pub fn distance_from_origin(&self) -> i32 {
        self.lat.abs() + self.lon.abs()
    }
}

pub fn solve() {
    let (ship1, ship2, _) = tools::read_lines("./input/day12.txt")
        .unwrap()
        .fold((Bearings::default(), Bearings::default(), Bearings::new(10, 1)),
              |(mut ship1, mut ship2, mut waypoint), line| {
                  let line = line.unwrap();
                  let command = line.chars().nth(0).unwrap();
                  let parameter = line[1..].parse::<i32>().unwrap();
                  match command {
                      'E'|'N'|'W'|'S' => {
                          ship1.shift_in_direction(command, parameter);
                          waypoint.shift_in_direction(command, parameter);
                      },
                      'F' => {
                          ship1.shift(parameter);
                          ship2.shift_to_waypoint(&waypoint, parameter);
                      },
                      'L'|'R' => {
                          ship1.turn(command, parameter);
                          waypoint.rotate(command, parameter);
                      },
                      _ => unreachable!(),
                  };
                  (ship1, ship2, waypoint)
        });
    println!("{}, {}", ship1.distance_from_origin(), ship2.distance_from_origin())
}