fn main() {
    let ipv4 = Address::IPV4(192,168,1,10);
    let ipv6 = Address::IPV6(
        String::from("FE80:CD00:0:CDDE:1257:0:211E:729C")
    );

    println!("{}", ipv4.to_string());
    println!("{}", ipv6.to_string());

    let ip3 = ipv4.clone();
}

#[derive(Debug)]
enum Address {
    IPV4(u8,u8,u8,u8),
    IPV6(String)
}

impl ToString for Address {
    fn to_string(&self) -> String {
        match self {
            Address::IPV4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            Address::IPV6(s) => s.clone()
        }
    }
}
