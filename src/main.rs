use std::env::args;
use std::process::Command;

fn main() {
    let mut args = args();
    println!("Path: {:?}", args.next().unwrap());
    let mut command = Command::new("7z");
    // loop {
    //     if let Some(a) = args.next() {
    //         if a == "--" {
    //             break;
    //         } else if a.starts_with("-") {
    //             // No command line options defined yet
    //             println!("Unexpected param {:?}", a);
    //         } else {
    //             println!("Unexpected param {:?}", a);
    //         }
    //     } else {
    //         break;
    //     }
    // }
    while let Some(a) = args.next() {
        println!("Arg: {:?}", a);
        if a.starts_with("-") && !a.starts_with("--") {
            // Special processing
            match &a[..] {
                "-o" => handle_dir(&mut command, &a, args.next()),
                "-p" | "-t" | "-v" | "-w" | "-x" => concat(&mut command, &a, args.next()),
                _ => {
                    command.arg(a);
                }
            }
        } else {
            command.arg(a);
        }
    }
    let status = command.status().expect("Failed to execute `7z`");
    std::process::exit(status.code().expect("Process closed by signal"));
}

fn handle_dir(command: &mut Command, begin: &str, param: Option<String>) {
    if let Some(param) = param {
        if param.starts_with("/") {
            command.arg(begin.to_owned() + &param);
        } else {
            command.arg(begin.to_owned() + "./" + &param);
        }
    } else {
        command.arg(begin);
    }
}
fn concat(command: &mut Command, begin: &str, param: Option<String>) {
    if let Some(param) = param {
        command.arg(begin.to_owned() + &param);
    } else {
        command.arg(begin);
    }
}
