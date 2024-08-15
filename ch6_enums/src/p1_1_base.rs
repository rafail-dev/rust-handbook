enum IpAddrKind1 {
    V4,
    V6,
}
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn main() {
    let ip11 = IpAddrKind1::V4;
    let _ip12 = IpAddrKind1::V6;

    match ip11 {
        IpAddrKind1::V4 => println!("ip11 is v4"),
        IpAddrKind1::V6 => println!("ip11 is v6"),
    }

    let ip21: IpAddrKind2 = IpAddrKind2::V4(192, 168, 0, 1);
    let _ip22: IpAddrKind2 = IpAddrKind2::V6(String::from("::1"));

    match ip21 {
        IpAddrKind2::V4(a, b, c, d) => println!("ip21 is v4 {}, {}, {}, {}", a, b, c, d),
        IpAddrKind2::V6(s) => println!("ip21 is v6 - {}", s),
    }
}
