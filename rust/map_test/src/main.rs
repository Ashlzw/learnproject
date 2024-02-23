use std::vec;

fn main() {
    println!("Hello, world!");

    let mut depart_map = std::collections::HashMap::new();

    let key = String::from("Engineering");
    let depart = vec![String::from("Sally")];

    depart_map.insert(key, depart);

    println!("{:?}", depart_map);
}
