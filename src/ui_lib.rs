use std::vec::{Vec};
use std::string::{String};

pub enum Conclusion {
    Fail,
    NoFlags,
    Flags,
}
pub enum FlagType {
    TTL,
    Interval,
    Count,
}


pub struct Flag{
    flag_type: FlagType,
    raw_value: String,
}
impl Flag{
    pub fn new(strarg: String) -> Flag{
        if !strarg.contains("=") {
            panic!("Flag not formated properly: {}",strarg);
        }
        let mut split = strarg.split("=").into_iter();
        let flag_string = split.nth(0).unwrap().to_string();
        let raw_value = split.nth(1).unwrap().to_string();
        Flag{
            flag_type: Flag::flag_type_from_string(flag_string).unwrap(),
            raw_value: raw_value,
        }
    }
    fn flag_type_from_string(str_flag: String) -> Result<FlagType,String>{
        let ttl = String::from("--ttl");
        let interval = String::from("--i");
        let count = String::from("--c");
        
        match str_flag.to_ascii_lowercase() {
            ttl => Ok(FlagType::TTL),
            interval => Ok(FlagType::Interval),
            count => Ok(FlagType::Count),
            _=> Err(format!("Invalid Flag Name: {}",str_flag)),
            
            
        }
    }
    
    
}

pub fn get_flags() -> Vec::<Flag>{
    let mut flag_vec = Vec::new();
    for argument in std::env::args() {
        flag_vec.push(Flag::new(argument));
    }
    flag_vec
}

pub fn arg_count() -> i32 {
    let mut arg_count = 0;
    std::env::args().for_each(|_| {
        arg_count+=1;
    });
    arg_count
}

pub fn check_correct_args() -> Conclusion {
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