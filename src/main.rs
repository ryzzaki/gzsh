use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn run_command(cmd: &str, args: &[&str]) {
    let child = Command::new(cmd).args(args).spawn();

    match child {
        Ok(mut child) => {
            child.wait().expect("Something went wrong, yo 🤔");
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn main() {
    loop {
        // using print instead of ln to output the carrot and the commands on the same line
        print!("👉 ");
        stdout().flush().expect("Sheesh 🥵");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // split the main command from the args
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "🏃" => {
                // implementation for cd
                let root = Path::new(args.iter().peekable().peek().map_or("/", |x| *x));
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "👋" => {
                // implementation for exit
                eprint!("Sayonara 🫡");
                return;
            }
            "📖" => run_command("ls", args),
            "🔎" => run_command("cat", args),
            "❌" => run_command("clear", args),
            "🗺" => run_command("pwd", args),
            "💀" => run_command("rm", args),
            "📦" => run_command("cargo", args),
            "😭" | "help" => {
                // implementation for help
                eprintln!("Help is not available.")
            }
            // fallback for now
            cmd => run_command(cmd, args),
        }
    }
}
