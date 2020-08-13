mod term;
mod tree;
mod zip;

pub use term::*;
pub use tree::*;
pub use zip::*;

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Output, Stdio};

fn main() {
    //comm();
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    /* Zip::extract_w_listfile(
        &Path::new(&args[1]),
        &Path::new(&args[2]),
        &Path::new(&args[3]),
    );
    */
    let z_dir = Zip::create_tree(&Path::new(&args[1]));
    println!("\n\nDONE\n");
    println!("{:#?}", z_dir);

    //let mut dir = Directory::new("$", "$");

    /*dir.add_dir("a", "a");
        dir.add_dir("a/b", "a/b");
        dir.add_dir("a/c", "a/c");
        dir.add_dir("a/b/d", "a/b/d");
        println!("{:#?}", dir);
        println!("{:#?}", dir.find_dir_mut("a/b/d"));
    */
}

fn ui() {
    let mut term = Term::new();
    term.ui_loop();
}

fn comm() {
    let out = Command::new("7z")
        .current_dir(Path::new(
            "/Users/onurcan/Code/Myself/7zip-browser/test/win",
        ))
        .arg("x")
        .arg("win.7z")
        //.arg("-i@list.txt")
        .arg("-o./out")
        .arg("-bsp1")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Cant Spawn Command");

    let mut sout = out.stdout.expect("stdout cant be read");
    let mut buf = [0u8; 2048];
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        sout.read(&mut buf);
        println!("{}", String::from_utf8_lossy(&buf));
    }
}
