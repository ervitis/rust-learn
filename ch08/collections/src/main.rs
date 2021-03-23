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
    println!("{:?}", v)
}
