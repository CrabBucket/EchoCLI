mod ui_lib;
use ui_lib::*;
use std::{vec};

fn main() {
    let mut flags = vec::Vec::new();
    match check_correct_args() {
        Conclusion::Fail => {
            panic!("Incorrect number of args");
        }
        Conclusion::NoFlags => {}
        
        Conclusion::Flags =>{
            flags = get_flags();
        }
    }
    for flag in flags{
        println!("{:?}",flag);
    }
}

