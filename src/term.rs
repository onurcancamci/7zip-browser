use crate::*;
use std::fs;
use std::io::{stdin, stdout, Read, Stdout, Write};
use std::path::Path;
use termion::color;
use termion::color::{Bg, Color, Fg, Reset};
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Term {
    _stdout: RawTerminal<Stdout>,
    root: Directory,
    out_dir: String,
    in_file: String,

    frame_start: usize,
    cursor: String,
    help_menu: bool,

    next_cursor: Option<String>,
    prev_cursor: Option<String>,
    will_down_move_frame: bool,
    will_up_move_frame: bool,
    is_cursor_dir: bool,
}

impl Term {
    pub fn new(root: Directory, in_file: String, out_dir: String) -> Self {
        Term {
            _stdout: stdout().into_raw_mode().unwrap(),
            cursor: root.get_name().to_owned(),
            root,
            frame_start: 0,
            next_cursor: None,
            prev_cursor: None,
            will_up_move_frame: false,
            will_down_move_frame: false,
            is_cursor_dir: true,
            out_dir,
            in_file,
            help_menu: false,
        }
    }
    pub fn draw(&mut self) {
        self.clear();
        let mut header: String = format!("  Help for '?'");
        let (w, h) = termion::terminal_size().unwrap();
        let space = " ".repeat((w as usize) - header.len());
        header = format!("{}{}", header, space);
        print!(
            "{}{}{}{}{}\r\n",
            Bg(color::White),
            Fg(color::Black),
            header,
            Bg(Reset),
            Fg(Reset)
        );

        print!("\r\n\r\n");

        if self.help_menu {
            println!("Up/Down or k/j to move\r\nEnter to open/close folders and mark files\r\n'm' to mark folders or files\r\n'e' to extract marked files and folders\r\nPress any key to close this menu\r");
            return;
        }

        let showns = self.root.shown_list();
        //calculate frame size
        let (_, ch) = self._stdout.cursor_pos().unwrap();
        let fh = (h - ch) - 3;
        if fh < 3 {
            self.clear();
            println!("Terminal Height Is Too Low. If Bugs Happen, Restart The Program\r");
            return;
        }
        //re-adjust frame start
        //if there is a space in the frame
        //adjsut frame
        if showns.len() - self.frame_start < fh as usize {
            self.frame_start = isize::max(0, (showns.len() as isize) - (fh as isize)) as usize;
            // TODO: cursor should be visible but make sure of that
        }
        if self.frame_start != 0 {
            println!("...\r");
        }

        let last_pos = usize::min(showns.len(), self.frame_start + fh as usize);
        for k in self.frame_start..last_pos {
            let el = &showns[k];
            let is_cursor = el.get_full_path() == self.cursor;
            if is_cursor {
                //set all flags
                if k == self.frame_start {
                    if self.frame_start != 0 {
                        panic!("Frame Start is not zero and cursor is at top");
                    }
                    self.will_up_move_frame = false;
                    self.prev_cursor = None;
                } else if k == self.frame_start + 1 {
                    //means cursor is 1 element below of first element
                    self.will_up_move_frame = self.frame_start > 0;
                    self.prev_cursor = Some((&showns[k - 1]).get_full_path().to_owned());
                } else {
                    self.will_up_move_frame = false;
                    self.prev_cursor = Some((&showns[k - 1]).get_full_path().to_owned());
                }

                if k == last_pos - 1 {
                    // above the last element
                    if k != showns.len() - 1 {
                        //if we are at bottom and we are not at last element,
                        //there is a problem in our logic
                        panic!("We are at bottom but there are more elements");
                    }
                    self.will_down_move_frame = false;
                    self.next_cursor = None;
                } else if k == last_pos - 2 {
                    //last element
                    self.will_down_move_frame = self.frame_start + (fh as usize) < showns.len();
                    self.next_cursor = Some((&showns[k + 1]).get_full_path().to_owned());
                } else {
                    self.will_down_move_frame = false;
                    self.next_cursor = Some((&showns[k + 1]).get_full_path().to_owned());
                }
            }
            if is_cursor {
                print!("{}{}", Bg(color::White), Fg(color::Black));
            } else {
                print!("{}{}", Bg(color::Reset), Fg(color::Reset));
            };
            match el {
                Node::File(f) => {
                    print!(
                        "{}[{}] {}",
                        "  ".repeat(f.get_level() as usize),
                        if f.is_marked() { "x" } else { " " },
                        f.get_name()
                    );
                    if is_cursor {
                        self.is_cursor_dir = false;
                    }
                }
                Node::Directory(d) => {
                    print!("{}{}/", "  ".repeat(d.get_level() as usize), d.get_name());
                    if is_cursor {
                        self.is_cursor_dir = true;
                    }
                }
            }
            print!("{}{}", Bg(color::Reset), Fg(color::Reset));
            print!("\r\n");
        }

        // k == last_pos - 1 should work too
        if self.frame_start + (fh as usize) < showns.len() {
            println!("...\r");
        }

        print!("{}", cursor::Hide);
        stdout().flush().unwrap();
    }
    fn clear(&mut self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn write_list_file(&self) -> String {
        let list = self.root.marked_list();
        let path_string = if Path::new("/tmp").exists() {
            format!("/tmp/.7zb-list-test.tmp")
        } else {
            format!("./.7zb-list-test.tmp")
        };
        let path = Path::new(&path_string);
        let mut file = fs::File::create(path).expect("Temporary list file creation error");
        for n in list {
            file.write(n.get_full_path().as_bytes())
                .expect("Temporary list write error");
            file.write("\n".as_bytes())
                .expect("Temporary list write error2");
        }
        path_string
    }

    pub fn ui_loop(&mut self) -> Option<String> {
        self.draw();
        //println!("{:#?}", self.next_cursor);
        for keycode in stdin().bytes() {
            let keycode = keycode.unwrap();
            self.help_menu = false;

            if keycode == 3 || keycode == 113 {
                print!("{}", cursor::Show);
                return None;
            }
            if keycode == 65 || keycode == 'k' as u8 {
                //up
                if self.will_up_move_frame {
                    self.frame_start -= 1;
                }
                if let Some(c) = &self.prev_cursor {
                    self.cursor = c.clone();
                }
            } else if keycode == 66 || keycode == 'j' as u8 {
                //down
                if self.will_down_move_frame {
                    self.frame_start += 1;
                }
                if let Some(c) = &self.next_cursor {
                    self.cursor = c.clone();
                }
            } else if keycode == 67 {
                //right
            } else if keycode == 68 {
                //left
            } else if keycode == 13 {
                //enter
                if self.is_cursor_dir {
                    self.root.toggle_open(&self.cursor);
                } else {
                    self.root.toggle_marked_file(&self.cursor);
                }
            } else if keycode == 'm' as u8 {
                //m
                if self.is_cursor_dir {
                    self.root.toggle_marked(&self.cursor);
                } else {
                    self.root.toggle_marked_file(&self.cursor);
                }
            } else if keycode == 'e' as u8 {
                self.clear();
                print!("{}", cursor::Show);
                self._stdout.flush().unwrap();
                let list_path = self.write_list_file();
                return Some(list_path);
            } else if keycode == '?' as u8 {
                self.help_menu = true;
            }
            self.draw();
            //println!("{}\r", keycode);
        }
        print!("{}", cursor::Show);
        None
    }
}
