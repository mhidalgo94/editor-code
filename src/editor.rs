use crate::Terminal;
use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal : Terminal,
    cursor_position : Position,
}


impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position {x:10, y:5},  
        }
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
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key){
        let Position {mut x , mut y} = self.cursor_position;

        match key  {
            Key::Up => y =  y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),          
        }
        self.cursor_position = Position {x, y}
    }

    fn refresh_screen(&self) -> Result<(), Error > {
        // print!("\x1b[2J"); // clear current output in terminal
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); // clear current output in terminal and position cursor
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position{ x:0, y:0});

        if self.should_quit{
            Terminal::clear_screen();
            println!("Closing Editor... \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();

        Terminal::flush()
    }

    fn draw_welcome_message(&self){
        let mut welcome_message = format!("Editor -- version {} \r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_add(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows (&self){
        let height = self.terminal.size().height - 1;
        for row in 0..height {
            Terminal::clear_current_line();
            println!("~\r");
            if row == height / 3 {
                self.draw_welcome_message();
                // let width = std::cmp::min(self.terminal.size().width as usize, msg_welcome.len());
                // println!("{}\r", &msg_welcome[..width]);
            } else {
                print!("~\r");
            }
        }
    }

}


fn die(e: Error){
    // print!("{}", termion::clear::All);
    Terminal::clear_screen();
    panic!("{}",e);
}