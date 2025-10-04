//read enum or struct.txt first

fn main() {
    //creating enum.
    enum IpAddrKind {
        V4,
        V6,
    }
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    //using enum in function
    fn route(_ip_kind: IpAddrKind){}
    route(_ip_kind:IpAddrKind::V4)
    route(_ip_kind:IpAddrKind::V6)

    //using enum with structs
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind : IpAddrKind::V4,
        address : String::from("127.0.0.1"),
    }

    let loopback = IpAddr {
        kind : IpAddrKind::V6,
        address : String::from("::1"),
    }

    //proper way of using enum.
    enum IpAddrKind2{
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let home 2 = IpAddrKind2::V4(String::from(127,0,0,1));
    let loopback = IpAddrKind2::V6(String::from("::1"));
}