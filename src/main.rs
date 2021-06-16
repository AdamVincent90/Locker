fn main() {
    // println!("Hello, Matches!");

    // let country_code = 100;

    // let country = match country_code {
    //     826 => "GB",
    //     46 => "SWE",
    //     0..=45 => "UNKNOWN",
    //     _ => "INVALID",
    // };

    // println!("country code {} is {}", country_code, country);

    // match_statement();

    combo_lock();
}

// fn match_statement() {
//     let mut i = 0;

//     while i <= 100 {
//         let num = match i {
//             0 => "Number is the default value of i",
//             1..=99 => "The number is currently between 1 and 99",
//             _ => "This number is too high so the match statement will end..",
//         };
//         println!("{}", num);
//         i += 1;
//     }
// }

// 3 stages of a lock = locked, unlocked, or invalid
enum Lock {
    LOCKED,
    UNLOCKED,
    INVALID,
}

fn combo_lock() {
    let code = String::from("Hello!"); // 24 bytes
    let mut locker = Lock::LOCKED;
    let mut attempt = 1;

    loop {
        match locker {
            Lock::LOCKED => {
                println!("Attempt {}", attempt);
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        if input.trim().eq(&code) {
                            locker = Lock::UNLOCKED;
                            continue;
                        } else {
                            println!("Incorrect Code");
                            attempt += 1;
                            if attempt > 5 {
                                locker = Lock::INVALID;
                            }
                        }
                    }
                    Err(_) => locker = Lock::INVALID,
                }
            }
            Lock::UNLOCKED => {
                println!("You are now logged in!");
                println!("Admin stuff can be set here!");
                break;
            }
            Lock::INVALID => {
                println!("Attempts exceeded.. exiting");
                break;
            }
        }
    }
}
