/* 1
enum IpAddreKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

/* 2
eum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
*/

/* PUT A STRUCT IN AN ENUM, WHY NOT?
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    //example constructors
    pub fn quit() -> Self {
        Message::Quit
    }

    pub fn mov(x: i32, y: i32) -> Self {
        Message::Move { x, y }
    }

    pub fn write<S: Into<String>>(text: S) -> Self {
        Message::Write(text.into())
    }

    pub fn change_color(r: i32, g: i32, b: i32) -> Self {
        Message::ChangeColor(r, g, b)
    }

    //helper queries
    pub fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    pub fn as_move(&self) -> Option<(i32, i32)> {
        match self {
            Message::Move { x, y } => Some((*x, *y)),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Message::Write(s) => Some(s.as_str()),
            _ => None,
        }
    }

    //processing/handling
    pub fn process(&self) {
        match self {
            Message::Quit => {
                println!("Recieved quit, shutting down...");
            }
            Message::Move { x, y } => {
                println!("Move to {x} {y}");
            }
            Message::Write(text) => {
                println!("Write: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change colors to: {} {} {}", r, g, b);
            }
        }
    }
}

fn main() {
    //1 let four = IpAddreKind::V4;
    //1 let six = IpAddreKind::V6;
    //2 let home = IpAddr::V4(127, 0, 0, 1);
    //2 let loopback = IpAddr::V6(String::from("::1"));
    let msgs = vec![
        Message::mov(10, 20),
        Message::write("Hello, Rust!"),
        Message::change_color(255, 128, 0),
        Message::quit(),
    ];

    for msg in msgs {
        //println!("> {msg:?}");
        msg.process();
        println!("---");
    }
}

//1 fn route(ip_kind: IpAddreKind) {}
