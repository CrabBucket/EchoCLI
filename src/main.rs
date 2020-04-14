enum Conclusion {
    Fail,
    TTLOn,
    TTLOff,
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
        1 => {
            println!("Expected 1 or 2 arguments found none please try one of the following: `EchoCLI <IP/Hostname>` or `EchoCLI <IP/Hostname> <TTL Time>`");
            Conclusion::Fail
        }
        2 => {
            Conclusion::TTLOff
        }
        3 => {
            Conclusion::TTLOn
        }
        _ => {
            println!("Expected 1 or 2 arguments please try one of the following: `EchoCLI <IP/Hostname>` or `EchoCLI <IP/Hostname> <TTL Time>`");
            Conclusion::Fail
        }
    }

}


fn main() {
    
}
