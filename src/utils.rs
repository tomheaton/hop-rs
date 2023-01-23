use std::collections::HashMap;

// TODO: use Rust-PHF for this?
/*pub const MULTIPLIERS: HashMap<&str, i64> = HashMap::from([
    ("B", 1),
    ("KB", 1024),
    ("MB", 1024 * 1024),
    ("GB", 1024 * 1024 * 1024),
]);*/

pub fn get_bytes(ram: &str) -> i64 {
    let multipliers: HashMap<&str, i64> = HashMap::from([
        // TODO: implement bytes (maybe check for "B" after checking hashmap?)
        // ("B", 1),
        ("KB", 1024),
        // TODO: is this broken?
        ("MB", 1024 * 1024),
        ("GB", 1024 * 1024 * 1024),
    ]);

    let ram = ram.to_uppercase();

    let unit = multipliers
        .iter()
        .find(|(&k, _)| ram.ends_with(k))
        .unwrap();

    let multiplier = multipliers.get(unit.0).unwrap().to_owned();
    let size = ram.replace(unit.0, "").parse::<i64>().unwrap();

    return size * multiplier;
}

// TODO: make id parsers here
