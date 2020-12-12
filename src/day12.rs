use crate::tools;

#[derive(Default)]
struct Bearings {
    lon: i32,
    lat: i32,
    heading: i32,
}

impl Bearings {
    pub fn new(lon: i32, lat: i32) -> Bearings {
        Bearings { lon, lat, heading: 0 }
    }

    pub fn turn(&mut self, clockwise: bool, angle: i32) {
        assert!(angle == 90 || angle == 180 || angle == 270);
        self.heading = (360 + self.heading + if clockwise {-1} else {1} * angle).abs() % 360;
    }

    pub fn rotate(&mut self, clockwise: bool, angle: i32) {
        let mut angle = angle;
        if !clockwise {
            angle = 360 - angle;
        }
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

    pub fn advance_with_heading(&mut self, heading: i32, distance: i32) {
        assert!(heading == 0 || heading == 90 || heading == 180 || heading == 270);
        match heading {
            0 =>   self.lon += distance, // east
            90 =>  self.lat += distance, // north
            180 => self.lon -= distance, // west
            270 => self.lat -= distance, // south
            _ => unreachable!(),
        };
    }

    pub fn advance(&mut self, distance: i32) {
        self.advance_with_heading(self.heading, distance);
    }

    pub fn move_to(&mut self, waypoint: &Bearings, repeat: i32) {
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
                      'E' => {
                          ship1.advance_with_heading(0, parameter);
                          waypoint.advance_with_heading(0, parameter);
                      },
                      'N' => {
                          ship1.advance_with_heading(90, parameter);
                          waypoint.advance_with_heading(90, parameter);
                      },
                      'W' => {
                          ship1.advance_with_heading(180, parameter);
                          waypoint.advance_with_heading(180, parameter);
                      },
                      'S' => {
                          ship1.advance_with_heading(270, parameter);
                          waypoint.advance_with_heading(270, parameter);
                      },
                      'F' => {
                          ship1.advance(parameter);
                          ship2.move_to(&waypoint, parameter);
                      },
                      'L' => {
                          ship1.turn(false, parameter);
                          waypoint.rotate(false, parameter);
                      },
                      'R' => {
                          ship1.turn(true, parameter);
                          waypoint.rotate(true, parameter);
                      },
                      _ => unreachable!(),
                  };
                  (ship1, ship2, waypoint)
        });
    println!("{}, {}", ship1.distance_from_origin(), ship2.distance_from_origin())
}