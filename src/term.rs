use std::io::{stdin, stdout, Read, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Term {
    _stdout: RawTerminal<Stdout>,
}

impl Term {
    pub fn new() -> Self {
        Term {
            _stdout: stdout().into_raw_mode().unwrap(),
        }
    }
    pub fn draw(&self) {
        //
    }
    fn clear(&mut self) {
        write!(self._stdout, "{esc}[2J{esc}[1;1H", esc = 27 as char).unwrap();
        self._stdout.flush().unwrap();
    }
    pub fn ui_loop(&mut self) {
        self.clear();
        for keycode in stdin().bytes() {
            let keycode = keycode.unwrap();
            if keycode == 3 || keycode == 113 {
                break;
            }
            println!("{}\r", keycode);
        }
    }
}
