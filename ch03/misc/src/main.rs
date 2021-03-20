
fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    for v in arr.iter() {
        print!("{}", v)
    }

    let mut n: [u32; 5] = [0; 5];
    for (k, v) in arr.iter().enumerate() {
        n[k] = v * 2;
    }
    another_fn(&mut n);

    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    let mut counter = 0;
    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3
        }
    };
    println!("counter result = {}", result);

    while result > 0 {
        result -= 2
    }
    println!("Final count = {}", result);

    for n in (1..5).rev() {
        print!("{}, ", n);
    }
}

fn another_fn(x: &mut [u32]) {
    for e in x.iter() {
        println!("{}", e)
    }
}
