use std::collections::VecDeque;
use std::ops::Add;

//pub enum Node {
//    Directory(Directory),
//    File(File),
//}

#[derive(Debug)]
pub struct Directory {
    c_dir: Vec<Directory>,
    c_file: Vec<File>,

    name: String,
    full_path: String,
}

#[derive(Debug)]
pub struct File {
    name: String,
    full_path: String,
}

impl File {
    pub fn new(name: &str, full_path: &str) -> Self {
        File {
            name: name.to_owned(),
            full_path: full_path.to_owned(),
        }
    }
}

impl Directory {
    pub fn new(name: &str, full_path: &str) -> Self {
        Directory {
            c_dir: vec![],
            c_file: vec![],
            name: name.to_owned(),
            full_path: full_path.to_owned(),
        }
    }

    //TODO: Add time and date
    //TODO: DOC: How it works how to use
    pub fn add_dir(&mut self, path: &str, f_path: &str) {
        let mut parts: VecDeque<&str> = path.split('/').collect();
        let c_name = parts
            .pop_front()
            .expect("Tree Creation Error, add_dir received broken path");
        let current_d = self.c_dir.iter_mut().find(|d| d.name == c_name);
        if let Some(current_d) = current_d {
            // Here parts length has to be longer than 0
            // the reason is creating files cant create directory
            // and directories cant be duplicate
            let mut n_path_str = String::with_capacity(path.len());
            parts.iter().enumerate().for_each(|(i, p)| {
                n_path_str.push_str(p);
                if i != parts.len() - 1 {
                    n_path_str.push_str("/");
                }
            });
            current_d.add_dir(&n_path_str, f_path);
        } else {
            self.c_dir.push(Directory::new(c_name, f_path));
        }
    }

    pub fn find_dir_mut(&mut self, path: &str) -> &mut Directory {
        let mut parts: VecDeque<&str> = path.split('/').collect();
        let c_name = parts
            .pop_front()
            .expect("Find Dir Error, find_dir_mut received broken path");
        let current_d = self
            .c_dir
            .iter_mut()
            .find(|d| d.name == c_name)
            .expect("Find Dir Error Path Does Not Exist");
        if parts.len() > 0 {
            let mut n_path_str = String::with_capacity(path.len());
            parts.iter().enumerate().for_each(|(i, p)| {
                n_path_str.push_str(p);
                if i != parts.len() - 1 {
                    n_path_str.push_str("/");
                }
            });
            current_d.find_dir_mut(&n_path_str)
        } else {
            current_d
        }
    }

    //TODO: add time, date and size
    pub fn add_file(&mut self, path: &str) {
        let mut parts: VecDeque<&str> = path.split('/').collect();
        let c_name = parts
            .pop_back()
            .expect("Add File Error, add_file received broken path");
        if parts.len() > 0 {
            let mut n_path_str = String::with_capacity(path.len());
            parts.iter().enumerate().for_each(|(i, p)| {
                n_path_str.push_str(p);
                if i != parts.len() - 1 {
                    n_path_str.push_str("/");
                }
            });
            let dir = self.find_dir_mut(&n_path_str);
            dir.c_file.push(File::new(c_name, path));
        } else {
            self.c_file.push(File::new(c_name, path));
        }
    }
}
