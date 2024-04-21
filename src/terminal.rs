use std::io::{self, stdin, stdout, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};



pub struct Size{
    pub width : u16,
    pub height: u16,
}

pub struct Terminal{
    size : Size,
    _stdout : RawTerminal<std::io::Stdout>,
}

impl Terminal{
    pub fn default()-> Result<Self, Error>{
        let size: (u16,u16) = termion::terminal_size()?;

        Ok(Self{
            size: Size{
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen(){
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x:u16, y:u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}",termion::cursor::Goto(x,y));
    }

    pub fn flush() -> Result<(), Error>{
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key,std::io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return  key;
            }
        }
    }


    pub fn size(&self) -> &Size{
        &self.size
    }
}