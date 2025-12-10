use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn closest(&self, others: &Vec<Coordinate>) -> Option<Coordinate> {
        others
            .iter()
            .min_by(|a, b| {
                let da = self.distance_to(a);
                let db = self.distance_to(a);

                da.partial_cmp(&db).unwrap()
            })
            .copied()
    }

    pub fn distance_to(&self, other: &Coordinate) -> f64 {
        let dx = f64::from(self.x - other.x).powi(2);
        let dy = f64::from(self.y - other.y).powi(2);
        let dz = f64::from(self.z - other.z).powi(2);

        (dx + dy + dz).sqrt()
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|t| !t.is_empty())
            .collect();

        if parts.len() < 3 {
            return Err("expected 3 integers in format x, y, z".into());
        }

        let x = parts.get(0).expect("Cound not get x coordinate");
        let x = x.parse::<i32>().expect("X coordinate is not a number");

        let y = parts.get(1).expect("Could not get y coordinate");
        let y = y.parse::<i32>().expect("Y coordinate is not a number");

        let z = parts.get(2).expect("Could not get z coordinate");
        let z = z.parse::<i32>().expect("Z coordinate is not a number");

        Ok(Self { x, y, z })
    }
}
