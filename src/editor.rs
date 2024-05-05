use crate::Terminal;
use std::io::Error;
use termion::event::Key;
use crate::{Document, Row};


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal : Terminal,
    document: Document,
    cursor_position : Position,
}


impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),  
            document : Document::default(),
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
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right 
            | Key::PageUp
            | Key::PageDown 
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key){
        let Position {mut x , mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_add(1) as usize;


        match key  {
            Key::Up => y =  y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width{
                    x = x.saturating_add(1);
                }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::End => x = width,
            Key::Home => y = 0,
            _ => (),          
        }
        self.cursor_position = Position {x, y}
    }

    fn refresh_screen(&self) -> Result<(), Error > {
        // print!("\x1b[2J"); // clear current output in terminal
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1)); // clear current output in terminal and position cursor
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

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

    fn draw_row(&self, row:&Row){
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    fn draw_rows (&self){
        let height = self.terminal.size().height - 1;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize){
                self.draw_row(row);
            }else if self.document.is_empty() &&  terminal_row == height / 3{
                self.draw_welcome_message();
            }else {
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