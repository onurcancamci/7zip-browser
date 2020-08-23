use std::collections::VecDeque;
use std::ops::Add;

#[derive(Debug)]
pub enum Node<'a> {
    Directory(&'a Directory),
    File(&'a File),
}

#[derive(Debug)]
pub struct Directory {
    c_dir: Vec<Directory>,
    c_file: Vec<File>,

    name: String,
    full_path: String,
    marked: bool,
    shown: bool,
    level: i32,
    child_shown: bool,
}

#[derive(Debug)]
pub struct File {
    name: String,
    full_path: String,
    marked: bool,
    shown: bool,
    level: i32,
}

pub trait FSNode {
    fn set_shown(&mut self, val: bool);
    fn get_level(&self) -> i32;
    fn is_marked(&self) -> bool;
    fn get_name(&self) -> &str;
    fn get_full_path(&self) -> &str;
    fn set_marked(&mut self, val: bool);
}

impl File {
    pub fn new(name: &str, full_path: &str, level: i32) -> Self {
        File {
            name: name.to_owned(),
            full_path: full_path.to_owned(),
            marked: false,
            shown: false,
            level,
        }
    }
}

impl FSNode for File {
    fn set_shown(&mut self, val: bool) {
        self.shown = val;
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_full_path(&self) -> &str {
        &self.full_path
    }

    fn set_marked(&mut self, val: bool) {
        self.marked = val;
    }
}

impl Directory {
    pub fn new(name: &str, full_path: &str, level: i32) -> Self {
        Directory {
            c_dir: vec![],
            c_file: vec![],
            name: name.to_owned(),
            full_path: full_path.to_owned(),
            marked: false,
            shown: false,
            level,
            child_shown: false,
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
            /*if self.name == "$" {
                println!("Dir {} {} {}", c_name, f_path, path);
            }*/
            self.c_dir
                .push(Directory::new(c_name, f_path, self.level + 1));
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

    pub fn find_file_mut(&mut self, path: &str) -> &mut File {
        let last_slash = path.rfind("/");
        let (dir, filename) = match last_slash {
            Some(last_slash) => {
                let (dir_path, filename) = path.split_at(last_slash);
                let filename = &filename[1..filename.len()]; //because "/" stays with filename
                (self.find_dir_mut(dir_path), filename)
            }
            None => {
                // file at root
                (self, path)
            }
        };
        dir.c_file
            .iter_mut()
            .find(|el| el.get_name() == filename)
            .expect("Cant Find File Selected, Error In System")
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
            dir.c_file.push(File::new(c_name, path, dir.level + 1));
        } else {
            self.c_file.push(File::new(c_name, path, self.level + 1));
        }
    }

    pub fn marked_list<'a>(&'a self) -> Vec<Node<'a>> {
        let mut v: Vec<Node<'a>> = vec![];
        for d in self.c_dir.iter() {
            if d.marked {
                v.push(Node::Directory(d));
            }
            let mut dir_res = d.marked_list();
            v.append(&mut dir_res);
        }
        for f in self.c_file.iter() {
            if f.marked {
                v.push(Node::File(f));
            }
        }
        v
    }

    pub fn shown_list<'a>(&'a self) -> Vec<Node<'a>> {
        let mut v: Vec<Node<'a>> = vec![];
        if self.shown {
            v.push(Node::Directory(self));
        }
        for d in self.c_dir.iter() {
            if d.shown {
                let mut dir_res = d.shown_list();
                v.append(&mut dir_res);
            }
        }
        for f in self.c_file.iter() {
            if f.shown {
                v.push(Node::File(f));
            }
        }
        v
    }

    pub fn set_shown_child(&mut self, val: bool) {
        for f in self.c_file.iter_mut() {
            f.set_shown(val);
        }
        for d in self.c_dir.iter_mut() {
            d.set_shown(val);
        }
    }

    pub fn set_shown_rec(&mut self, val: bool, initial: bool) {
        if !initial {
            self.set_shown(false);
        }
        for f in self.c_file.iter_mut() {
            f.set_shown(val);
        }
        for d in self.c_dir.iter_mut() {
            d.set_shown_rec(val, false);
        }
    }

    pub fn set_marked_rec(&mut self, val: bool) {
        self.set_marked(val);
        for f in self.c_file.iter_mut() {
            f.set_marked(val);
        }
        for d in self.c_dir.iter_mut() {
            d.set_marked_rec(val);
        }
    }

    pub fn open(&mut self, path: &str) {
        let dir = self.find_dir_mut(path);
        dir.set_shown_child(true);
    }

    pub fn close(&mut self, path: &str) {
        let dir = self.find_dir_mut(path);
        dir.set_shown_rec(false, true);
    }

    pub fn toggle_open(&mut self, path: &str) {
        let dir = if path == "$" {
            self
        } else {
            self.find_dir_mut(path)
        };
        if dir.child_shown {
            dir.set_shown_rec(false, true);
            dir.child_shown = false;
        } else {
            dir.set_shown_child(true);
            dir.child_shown = true;
        }
    }

    pub fn toggle_marked(&mut self, path: &str) {
        let dir = if path == "$" {
            self
        } else {
            self.find_dir_mut(path)
        };
        if dir.marked {
            dir.set_marked_rec(false);
        } else {
            dir.set_marked_rec(true);
        }
    }

    pub fn toggle_marked_file(&mut self, path: &str) {
        let f = self.find_file_mut(path);
        f.set_marked(!f.is_marked());
    }
}

impl FSNode for Directory {
    fn set_shown(&mut self, val: bool) {
        self.shown = val;
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_full_path(&self) -> &str {
        &self.full_path
    }

    fn set_marked(&mut self, val: bool) {
        self.marked = val;
    }
}

impl<'a> FSNode for Node<'a> {
    fn get_full_path(&self) -> &str {
        match self {
            Node::File(v) => v.get_full_path(),
            Node::Directory(v) => v.get_full_path(),
        }
    }

    fn set_shown(&mut self, _: bool) {
        panic!("Unspported operation");
    }

    fn set_marked(&mut self, _: bool) {
        panic!("Unspported operation");
    }

    fn get_name(&self) -> &str {
        match self {
            Node::File(v) => v.get_name(),
            Node::Directory(v) => v.get_name(),
        }
    }

    fn is_marked(&self) -> bool {
        match self {
            Node::File(v) => v.is_marked(),
            Node::Directory(v) => v.is_marked(),
        }
    }
    fn get_level(&self) -> i32 {
        match self {
            Node::File(v) => v.get_level(),
            Node::Directory(v) => v.get_level(),
        }
    }
}
