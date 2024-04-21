use crate::Terminal;
use std::io::Error;
use termion::event::Key;


pub struct Editor {
    should_quit: bool,
    terminal : Terminal,
}


impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal")  }
    }

    pub fn run(&mut self) {

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
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('q') => panic!("Program end"),
            Key::Ctrl('c') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error > {
        // print!("\x1b[2J"); // clear current output in terminal
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); // clear current output in terminal and position cursor
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);

        if self.should_quit{
            println!("Closing Editor. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::flush()
    }

    fn draw_rows (&self){
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }

}


fn die(e: Error){
    // print!("{}", termion::clear::All);
    Terminal::clear_screen();
    panic!("{}",e);
}