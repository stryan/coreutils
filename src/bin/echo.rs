use std::env;

fn main() {
    let mut nflag = false;
    let mut args = env::args();

    match args.next() {
        Some(s) => {
            if s == "-n" {
                nflag = true;
            } else {
                print!("{}",s)
            }
        },
        None => print!("")
    }

    loop {
        match args.next() {
            Some(s) => print!("{}",s),
            None => break
        }
    }

    if !nflag {
        println!("")
    }
        
}
