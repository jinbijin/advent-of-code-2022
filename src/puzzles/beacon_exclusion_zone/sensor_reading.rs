use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::common::{
    interval::{Interval, IntervalUnion},
    position::Position,
};

pub enum ParseSensorReadingError {
    InvalidFormat,
    InvalidCoordinate { input: String },
}

impl Display for ParseSensorReadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidCoordinate { input } => write!(f, "'{}' is not a valid coordinate", input),
        }
    }
}

impl Debug for ParseSensorReadingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseSensorReadingError {}

pub struct SensorReading {
    pub sensor: Position<isize>,
    pub beacon: Position<isize>,
}

impl FromStr for SensorReading {
    type Err = ParseSensorReadingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(": ") {
            None => Err(Self::Err::InvalidFormat),
            Some((sensor, beacon)) => {
                let sensor_prefix = "Sensor at x=";
                if !sensor.starts_with(sensor_prefix) {
                    return Err(Self::Err::InvalidFormat);
                }
                let sensor = match sensor[(sensor_prefix.len())..].split_once(", y=") {
                    None => Err(Self::Err::InvalidFormat),
                    Some((x, y)) => {
                        let x = x
                            .parse::<isize>()
                            .map_err(|_| Self::Err::InvalidCoordinate {
                                input: x.to_string(),
                            })?;
                        let y = y
                            .parse::<isize>()
                            .map_err(|_| Self::Err::InvalidCoordinate {
                                input: x.to_string(),
                            })?;
                        Ok(Position { x, y })
                    }
                }?;

                let beacon_prefix = "closest beacon is at x=";
                if !beacon.starts_with(beacon_prefix) {
                    return Err(Self::Err::InvalidFormat);
                }
                let beacon = match beacon[(beacon_prefix.len())..].split_once(", y=") {
                    None => Err(Self::Err::InvalidFormat),
                    Some((x, y)) => {
                        let x = x
                            .parse::<isize>()
                            .map_err(|_| Self::Err::InvalidCoordinate {
                                input: x.to_string(),
                            })?;
                        let y = y
                            .parse::<isize>()
                            .map_err(|_| Self::Err::InvalidCoordinate {
                                input: x.to_string(),
                            })?;
                        Ok(Position { x, y })
                    }
                }?;

                Ok(SensorReading { sensor, beacon })
            }
        }
    }
}

impl SensorReading {
    pub fn slice_horizontal(&self, y: isize) -> Option<Interval<isize>> {
        let sensor_beacon = self.sensor.manhattan_distance(self.beacon);
        let sensor_slice: isize = match self.sensor.y.abs_diff(y).try_into() {
            Ok(distance) => distance,
            Err(_) => {
                return None;
            }
        };
        let overlap = sensor_beacon - sensor_slice;
        if overlap >= 0 {
            match Interval::build(self.sensor.x - overlap, self.sensor.x + overlap + 1) {
                Ok(interval) => Some(interval),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub fn get_covered_position_count(readings: Vec<SensorReading>, y: isize) -> isize {
    let slice = get_slice(&readings, y);

    let beacons = readings
        .iter()
        .map(|x| x.beacon)
        .collect::<HashSet<Position<isize>>>();
    let beacons_in_range = beacons
        .iter()
        .filter(|p| p.y == y && slice.contains(p.x))
        .count() as isize;

    slice.count() - beacons_in_range
}

pub fn scan(readings: Vec<SensorReading>, scale: isize) -> isize {
    let interval = match Interval::build(0, scale * 2) {
        Ok(interval) => interval,
        Err(_) => {
            return -1;
        }
    };
    for y in 0..=(scale * 2) {
        let slice = get_slice(&readings, y).overlap(interval);
        if slice.0.len() == 2 {
            for p in slice.0.iter() {
                if p.start() == 0 {
                    return p.end() * 4000000 + y;
                }
            }
        }
    }
    return -1;
}

fn get_slice(readings: &Vec<SensorReading>, y: isize) -> IntervalUnion<isize> {
    let mut slice: IntervalUnion<isize> = IntervalUnion::new();

    for reading in readings.iter() {
        if let Some(interval) = reading.slice_horizontal(y) {
            slice.add(&interval);
        }
    }

    slice
}
