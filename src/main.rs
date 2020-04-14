mod ui_lib;
use ui_lib::*;


fn main() {
    let mut TTL_status = false;
    let mut TTL_value = 0u32;
    match check_correct_args() {
        Conclusion::Fail => {

        }
        Conclusion::NoFlags =>{

        }
        Conclusion::Flags =>{
            TTL_value = u32::from_str_radix(&std::env::args().nth(2).unwrap(),10).unwrap();
            TTL_status = true;
        }
    }
}

fn arg_count() -> i32 {
    let mut arg_count = 0;
    std::env::args().for_each(|_| {
        arg_count+=1;
    });
    arg_count
}

fn check_correct_args() -> Conclusion {
    let arg_count = arg_count();
    
    match arg_count{
        std::i32::MIN..=0 => {
            println!("Invalid arguments, negative amount of arguments received, try the following format: `EchoCLI <IP/Hostname>` or `EchoCLI <IP/Hostname> <--option=value>`");
            Conclusion::Fail
        }
        1 => {
            println!("Expected arguments found none please try the following format: `EchoCLI <IP/Hostname>` or `EchoCLI <IP/Hostname> <--option=value>`");
            Conclusion::Fail
        }
        2 => {
            Conclusion::NoFlags
        }
        3..=std::i32::MAX => {
            Conclusion::Flags
        }
    }

}