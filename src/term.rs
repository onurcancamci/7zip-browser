use crate::*;
use std::io::{stdin, stdout, Read, Stdout, Write};
use termion::color;
use termion::color::{Bg, Color, Fg, Reset};
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Term {
    _stdout: RawTerminal<Stdout>,
    root: Directory,
    frame_start: usize,
    cursor: String,

    next_cursor: Option<String>,
    prev_cursor: Option<String>,
    will_down_move_frame: bool,
    will_up_move_frame: bool,
}

impl Term {
    pub fn new(root: Directory) -> Self {
        Term {
            _stdout: stdout().into_raw_mode().unwrap(),
            cursor: root.get_name(),
            root,
            frame_start: 0,
            next_cursor: None,
            prev_cursor: None,
            will_up_move_frame: false,
            will_down_move_frame: false,
        }
    }
    pub fn draw(&mut self) {
        self.clear();
        let mut header: String = format!("  Header");
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
                    self.prev_cursor = Some((&showns[k - 1]).get_full_path());
                } else {
                    self.will_up_move_frame = false;
                    self.prev_cursor = Some((&showns[k - 1]).get_full_path());
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
                    self.next_cursor = Some((&showns[k + 1]).get_full_path());
                } else {
                    self.will_down_move_frame = false;
                    self.next_cursor = Some((&showns[k + 1]).get_full_path());
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
                }
                Node::Directory(d) => {
                    print!("{}{}/", "  ".repeat(d.get_level() as usize), d.get_name());
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
        //write!(self._stdout, "{esc}[2J{esc}[1;1H", esc = 27 as char).unwrap();
        //self._stdout.flush().unwrap();
    }
    pub fn ui_loop(&mut self) {
        self.draw();
        //println!("{:#?}", self.next_cursor);
        for keycode in stdin().bytes() {
            let keycode = keycode.unwrap();
            if keycode == 3 || keycode == 113 {
                print!("{}", cursor::Show);
                break;
            }
            if keycode == 65 {
                //up
                if self.will_up_move_frame {
                    self.frame_start -= 1;
                }
                if let Some(c) = &self.prev_cursor {
                    self.cursor = c.clone();
                }
            } else if keycode == 66 {
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
            }
            self.draw();
            //println!("{}\r", keycode);
        }
    }
}
