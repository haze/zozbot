
extern crate rand;
use rand::Rng;

extern crate irc;
use irc::client::prelude::*;

fn zozzes(n: u32) -> String {

    fn get_zoz() -> String {
        let mut rng = rand::thread_rng();
        rng.choose(vec!["tip top", "zippity", "zoopity", "bib-bop-bong", "zongba", "zim-zam", "flim-flam", "fozzle-cozzle", "zozzle", "zozzle", "zozzle", "zozzle", "zozzle-mozzle", "fozzle", "bippity bap", "moobity", "zobby", "fabby", "flabby", "commily", "fommily", "zozzle", "wozzle", "flop", "floppity", "mibby mibby", "zibby zibby", "zim zam zom zopple", "zopple", "zapple", "zoozle"].as_slice() ).unwrap().to_string()
    }

    let mut x: String = String::new();
    for z in 0 .. n {
        x.push_str(&*get_zoz());
        if z > 0 {
            x.push(' ');
        }
    }
    x
}

fn main() {
    let server: IrcServer = IrcServer::new("conf/config.toml").unwrap();
    server.identify().unwrap();
    server.for_each_incoming(|message: Message| {
        match message.command {
            Command::PRIVMSG(ref target, ref message) => {
                if message.starts_with(".zoz") {
                    let after = message.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
                    let num = if after.parse::<u32>().is_ok() { after.parse::<u32>().ok().unwrap() } else { 1 };
                    server.send_privmsg(target, &*zozzes(if num > 300 { 1 } else { num })).ok();
                }
            }
            _ => ()
        }
    }).unwrap();
}