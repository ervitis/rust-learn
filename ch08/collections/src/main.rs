use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(5);
    v.push(6);

    println!("{:?}", v);

    let another_v = vec![1, 2, 3];

    println!("{:?}", another_v);

    fn hello_there(vec: &mut Vec<i32>) {
        vec.pop();
    }

    hello_there(&mut v);
    println!("{:?}", v);

    for value in v {
        println!("{}", value);
    }

    for value in another_v {
        println!("{}", value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let another_scores: HashMap<_, _> = vec![String::from("Blue"), String::from("Yellow")]
        .into_iter()
        .zip(vec![10, 50].into_iter())
        .collect();

    println!("{:?} and {:?}", scores, another_scores);
    println!("Blue score is {:?}", scores.get(&String::from("Blue")));

    for (k, v) in &scores {
        println!("key {}: has value {}", k, v);
    }

    scores.insert(String::from("Yellow"), 34);
    println!("updating value {:?}", scores.get(&String::from("Yellow")));

    scores.entry(String::from("Red")).or_insert(32);
    println!("new entry {:?}", scores.get(&String::from("Red")));
}
