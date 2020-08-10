//pub enum Node {
//    Directory(Directory),
//    File(File),
//}

pub struct Directory {
    c_dir: Vec<Directory>,
    c_file: Vec<File>,
}

pub struct File {
    fpath: String,
}
