use crate::*;
use std::path::Path;

pub struct Zip;

impl Zip {
    pub fn extract_w_listfile(zip_file: &Path, out_dir: &Path, list_file: &Path) {
        let mut out = Command::new("7z")
            .arg("x")
            .arg(zip_file)
            .arg(format!(
                "-i@{}",
                list_file.to_str().expect("List File Path Is Invalid")
            ))
            .arg(format!(
                "-o{}",
                out_dir.to_str().expect("Out Dir Path Is Invalid")
            ))
            .arg("-bsp1")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Cant Spawn Command");

        out.wait().expect("Process Wait Error");
        //let mut sout = out.stdout.expect("stdout cant be read");
    }
}
