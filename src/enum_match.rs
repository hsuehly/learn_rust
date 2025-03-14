enum IpAddrKind {
    V4,
    V6,
}
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(u) = config_max {
        println!("The maximum is configured to be {}", u);
    }
}
