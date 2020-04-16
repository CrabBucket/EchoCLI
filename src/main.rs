mod ui;
mod network;
use ui::*;
use std::{vec};

fn main() {
    //print!("{:b}",u16::from(true) << 15);
    network::test();
        // let mut flags = vec::Vec::new();
        // match check_correct_args() {
        //     Conclusion::Fail => {
        //         panic!("Incorrect number of args");
        //     }
        //     Conclusion::NoFlags => {}
            
        //     Conclusion::Flags =>{
        //         flags = get_flags();
        //     }
        // }
        // for flag in flags{
        //     println!("{:?}",flag);
        // }
}

