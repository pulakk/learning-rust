use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("water"), 0);
    map.insert(String::from("bottle"), 100);

    println!("{:?}", map);

    /* map is a vector of tuples */
    let map: Vec<(i32, i32)> = vec![0, 100]
        .into_iter()
        .zip(vec![0, 100].into_iter())
        .collect();
    println!("{:?}", map);

    /* map is a hashmap */
    let map: HashMap<_, _> = vec![0, 100]
        .into_iter()
        .zip(vec![0, 100].into_iter())
        .collect();
    println!("{:?}", map);


    // OWNERSHIP
    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();

    // map.insert(key, value); // moves "key" and "value"
    map.insert(&key, &value);

    println!("{}: {}", key, value);
    println!("The value of {} is {}", &key, map[&key]);
    println!("The value of {} is {}", &key, map.get(&key).unwrap()); // returns Option

    // LOOP
    let mut map = HashMap::new();
    map.insert(String::from("water"), 10);
    map.insert(String::from("bottle"), 20);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Updating
    map.insert(String::from("water"), 12);
    println!("{:?}", map); // {"bottle": 20, "water": 12}

    map.entry(String::from("water")).or_insert(10); // insert if empty
    println!("{:?}", map); // {"bottle": 20, "water": 12}

    let mut map = HashMap::new();
    let text = "a quick brown fox jumps over the lazy dog";
    for letter in text.chars() {
        *(map.entry(letter).or_insert(0)) += 1; // frequency of each letter
    }
    println!("{:?}", map);
}
