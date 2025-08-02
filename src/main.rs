use std::process::Command;

#[cfg(not(windows))]
compile_error!("This program is only meant for Windows.");

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return;
    }

    let jar = &args[1];

    Command::new("java")
        .args(&["-jar", jar])
        .status()
        .expect("could not run jar");
}
