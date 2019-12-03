use std::collections::HashSet;

#[derive(Eq)]
struct Coordinates {
    point: (i32, i32),
    step: i32,
}
impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        self.point == other.point
    }
}

impl std::hash::Hash for Coordinates {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl std::borrow::Borrow<(i32, i32)> for Coordinates {
    fn borrow(&self) -> &(i32, i32) {
        &self.point
    }
}
fn gen_coords(coords: &mut HashSet<Coordinates>, wire: &Vec<&str>) {
    let mut point = (0, 0);
    let mut step = 0;
    for &token in wire.iter() {
        let mut chars = token.chars();
        let cur: i32 = token[1..].parse().unwrap();
        let ch = chars.nth(0).unwrap();
        for _ in 0..cur {
            match ch {
                'R' => {
                    point.0 += 1;
                }
                'L' => {
                    point.0 -= 1;
                }
                'U' => {
                    point.1 += 1;
                }
                'D' => {
                    point.1 -= 1;
                }
                _ => {}
            }
            step += 1;
            coords.insert(Coordinates { point, step });
        }
    }
}

fn main() {
    let data: Vec<&str> = include_str!("../input").split('\n').collect();
    let wire1: Vec<&str> = data[0].split(',').collect();
    let wire2: Vec<&str> = data[1].split(',').collect();
    let mut coords1: HashSet<Coordinates> = HashSet::new();
    let mut coords2: HashSet<Coordinates> = HashSet::new();

    gen_coords(&mut coords1, &wire1);
    gen_coords(&mut coords2, &wire2);

    let inte: HashSet<_> = coords1.intersection(&coords2).collect();

    let distance = inte
        .iter()
        .map(|coord| i32::abs(coord.point.0) + i32::abs(coord.point.1))
        .min();
    let steps = inte
        .iter()
        .map(|coord| {
            coords1.get(&coord.point).unwrap().step + coords2.get(&coord.point).unwrap().step
        })
        .min();
    println!("{:?} {:?}", distance, steps);
}
