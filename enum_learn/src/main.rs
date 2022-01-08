// enum IPAddrKind {
//     V4,
//     V6
// }
enum IPAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}
struct IPAddr {
    kind: IPAddrKind,
    address: String
}
fn main() {
    let four = IPAddrKind::V4(127,0,0,1);
    let six = IPAddrKind::V6(String::from("::1"));
    route(four);
    route(six);
}

fn route(ip_kind: IPAddrKind) {

}
