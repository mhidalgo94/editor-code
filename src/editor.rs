use crate::Terminal;
use std::io::Error;
use termion::event::Key;
use crate::{Document, Row};
use std::env;
use termion::color;
use std::time::Duration;
use std::time::Instant;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(238,238,238);
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct StatusMessage {
    text:String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self{
            time: Instant::now(),
            text: message
        }
    }
}




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
    offset: Position,
    status_message : StatusMessage,
}


impl Editor {
    pub fn default() -> Self {
        let args:Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-C = quit");

        let document = if args.len() > 1{
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok(){
                doc.unwrap()
            }else{
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else{
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),  
            offset: Position::default(),
            document,
            status_message : StatusMessage::from(initial_status),
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
        self.scroll();
        Ok(())
    }

    fn move_cursor(&mut self, key: Key){
        let terminal_heigth = self.terminal.size().height as usize; 
        let Position {mut x , mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let mut width = if let Some(row) = self.document.row(y){
            row.len()
        } else{
            0
        };
        
        match key  {
            Key::Up => y =  y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                }else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y){
                        x = row.len()
                    }else {
                        x = 0
                    }
                }
            },
            Key::Right => {
                if x < width{
                    x += 1;
                } else if  y < height{
                    y+=1;
                    x = 0;
                }
            },
            Key::PageUp => {
                y = if y > terminal_heigth{
                    y - terminal_heigth
                }else{
                    0
                }
            },
            Key::PageDown => {
                y = if y.saturating_add(terminal_heigth) < height{
                    y + terminal_heigth as usize
                } else{
                    height
                }
            },
            Key::End => x = width,
            Key::Home => y = 0,
            _ => (),          
        }

        width = if let Some(row)  = self.document.row(y){
            row.len()
        }else {
            0
        };
        if x > width {
            x  = width;
        }
        self.cursor_position = Position {x, y}
    }

    fn scroll(&mut self){
        let Position {x,y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.document.len();
        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height){
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width){
            offset.x = x.saturating_sub(width).saturating_add(1);
        }

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
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
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
        // let start = 0;
        // let end = self.terminal.size().width as usize;
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    
    fn draw_rows (&self){
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y){
                self.draw_row(row);
            }else if self.document.is_empty() &&  terminal_row == height / 3{
                self.draw_welcome_message();
            }else {
                print!("~\r");
            }
        }
    }

    fn draw_status_bar(&self){
        // let spaces = " ".repeat(self.terminal.size().width as usize);
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name  = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name{
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!("{} - {} lines", file_name, self.document.len());
        // if width > status.len(){
        //     status.push_str(&" ".repeat(width - status.len()));
        // }

        let line_indicator = format!(
            "{}/{}",self.cursor_position.y.saturating_add(1),self.document.len()
        );

        let len = status.len() + line_indicator.len();

        if width > len {
            status.push_str(&" ".repeat(width - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_bg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self){
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5,0){
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
}


fn die(e: Error){
    // print!("{}", termion::clear::All);
    Terminal::clear_screen();
    panic!("{}",e);
}