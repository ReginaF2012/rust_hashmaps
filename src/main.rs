fn main() {
    // bring hashmaps into scope
    use std::collections::HashMap;

    // create hashmap
    let mut scores = HashMap::new();

    // insert a key of "Blue" and a value of 10
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

    // create a hashmap using collect and zip
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let team_name = String::from("Blue");
        
        // using .get() to get a value in a hashmap
        let score = scores.get(&team_name);

        // iterate over a hasmap
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // overwriting a value
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{:?}", scores);

        // only insert if a key has no value

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
    
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{:?}", scores);

        // updating based on old value

            let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
}
