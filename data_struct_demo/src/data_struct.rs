use std::collections::HashMap;

pub fn vec_test() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    //v.push(6);
    println!("the first value is {}", first);
}

pub fn vec_for() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

pub fn vec_var() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row);
    for i in &mut row {
        println!("{:#?}", *i);
    }
}

pub fn str_add() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

}

pub fn hash_map_demo() {
    let mut scores = HashMap::new();  
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("entry is {:#?}", scores.entry(String::from("Yellow")));
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("scores is {:#?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let tmp = teams.iter().zip(initial_scores.iter());
    println!("tmp value {:#?}", tmp);
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("hashmap value {:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("map value {:#?}", map);
    println!("field_name={} field_value={}", field_name, field_value);

    for (key, value) in &map {
        println!("key={} value={}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
