use std::io::{self, stdin, stdout, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
// use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    pub should_quit: bool,
}


impl Editor {

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keyress(){
                die(error);
            }
        }
        // for key in io::stdin().keys(){
        //     match key {
        //         Ok(key) => match key {
        //             Key::Char(c)=>{
        //                 if c.is_control(){
        //                     println!("first if: {:?}\r", c as u8);
        //                 }else {
        //                     println!("the else {:?} ({})\r", c as u8,c);

        //                 }
        //             }
        //             Key::Ctrl('c') => break,
        //             // Key::Alt('c') => break,
        //             _ => println!("{key:?}\r"),
        //         }
        //         Err(err) => die(err),
        //     }
        // }
    }

    fn process_keyress(&mut self) -> Result<(), Error>{
        let pressed_key = read_key()?;
        match pressed_key {
            // Key::Ctrl('q') => panic!("Program end"),
            Key::Ctrl('c') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    pub fn default() -> Self {
        Self{ should_quit: false}
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        // print!("\x1b[2J"); // clear current output in terminal
        print!("{}", termion::clear::All);
        io::stdout().flush()
    }

}

fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: Error){
    panic!("{}",e);
}