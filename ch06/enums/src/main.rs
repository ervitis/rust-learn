
enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let home = IpAddr {
        address: String::from("0.0.0.0"),
        kind: IpAddrKind::V4(String::from("0.0.0.0")),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1")
    };

    let n = Some(5);
    let absent: Option<i32> = None;

    println!("sum {} and 5 is={}", n.unwrap(), n.unwrap() + 5);
}
