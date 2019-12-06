use std::collections::HashMap;

fn trace_path<'a>(orbits: &'a HashMap<&str, &str>, start: &str) -> Vec<&'a str> {
    let mut satetlite = start;
    let mut store = Vec::new();
    while let Some(sat) = orbits.get(satetlite) {
        satetlite = sat;
        store.push(*sat)
    }
    store
}

fn main() {
    let inputs: Vec<&str> = include_str!("../input").split('\n').collect();

    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for x in inputs.iter() {
        let data: Vec<&str> = x.split(')').collect();
        orbits.insert(data[1], data[0]);
    }

    let ypos: Vec<&str> = trace_path(&orbits, "YOU");
    let spos: Vec<&str> = trace_path(&orbits, "SAN");
    for (x, mine) in ypos.iter().enumerate() {
        for (y, santa) in spos.iter().enumerate() {
            if mine == santa {
                println!("distance {}", x + y);
                return;
            }
        }
    }
}
