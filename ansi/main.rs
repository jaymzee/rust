use std::num::ParseIntError;

fn main() {
    parse_ansi_str("café\x1b[1;31mred\x1b[mλ");
}

#[derive(Debug, Copy, Clone)]
enum Ansi {
    Char,   // regular characters
    Esc,    // in an escape sequence
    Csi,    // in a Control Sequence Introducer
}

fn parse_ansi_str(s: &str) {
    let mut state = Ansi::Char;
    let mut arg = 0;

    for (i, c) in s.char_indices() {
        let next_state = match (state, c) {
            (Ansi::Char, '\u{1b}') => {
                Ansi::Esc
            }
            (Ansi::Char, _) => {
                print_char(c);
                Ansi::Char
            }
            (Ansi::Esc, '[') => {
                arg = i + 1;
                Ansi::Csi
            }
            (Ansi::Csi, '\u{20}'..='\u{3f}') => {
                // parameters and intermediate bytes
                Ansi::Csi
            }
            (Ansi::Csi, '\u{40}'..='\u{7E}') => {
                // final byte
                csi(c, &s[arg..i]);
                Ansi::Char
            }
            _ => panic!("failed to parse ansi sequence")
        };
        //println!("{:x}: state: {:?} -> {:?}", c as u32, state, next_state);
        state = next_state;
    }
}

fn csi(n: char, args: &str) {
    match n {
        'm' => sgr(args),
        'H' => {
            match split(args, ';').as_slice() {
                [Ok(r), Ok(c)] => println!("[CUP {} ; {}]", *r, *c),
                _ => println!("invalid CUP arguments"),
            }
        }
        _ => println!( "unsupported ansi sequence CSI {}", n)
    }
}

fn sgr(args: &str) {
    if args == "" || args == "0" {
        print!("[SGR RESET]");
    } else {
        for code in split(args, ';') {
            match code {
                Ok(1) => print!("[SGR INTENSITY]"),
                Ok(n) if (30..=37).contains(&n) =>
                    print!("[SGR FG {}]", n - 30),
                Ok(n) if (40..=47).contains(&n) =>
                    print!("[SGR BG {}]", n - 40),
                Ok(n) => println!("invalid SGR code {}", n),
                Err(e) => println!("failed to parse SGR arguments: {}", e)
            }
        }
    }
}

fn split(args: &str, delimiter: char) -> Vec<Result<u8, ParseIntError>> {
    args.split(delimiter)
        .map(|s| s.parse())
        .collect()
}

fn print_char(c: char) {
    print!("{}", c);
}


