use std::env;
use std::fs::File;
use std::io::Write;
mod chip8;
use std::time::Instant;
use std::{io::{self, Stdout}, thread, time::Duration, error::Error};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{self, KeyCode, Event, EnableMouseCapture}};
use ratatui::style::Style;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, canvas::{Canvas, Map, MapResolution, Line, Rectangle, Points}},
    Terminal, style::Color, symbols::Marker
};
use ratatui::widgets::canvas::Context;


 const INTRO:[u8;246] = [106, 2, 107, 12, 108, 63, 109, 12, 162, 234, 218, 182, 220,
    214, 110, 0, 34, 212, 102, 3, 104, 2, 96, 96, 240, 21, 240, 7, 48, 0,
    18, 26, 199, 23, 119, 8, 105, 255, 162, 240, 214, 113, 162, 234, 218,
    182, 220, 214, 96, 1, 224, 161, 123, 254, 96, 4, 224, 161, 123, 2, 96,
    31, 139, 2, 218, 182, 141, 112, 192, 10, 125, 254, 64, 0, 125, 2, 96, 0,
    96, 31, 141, 2, 220, 214, 162, 240, 214, 113, 134, 132, 135, 148, 96,
    63, 134, 2, 97, 31, 135, 18, 70, 2, 18, 120, 70, 63, 18, 130, 71, 31,
    105, 255, 71, 0, 105, 1, 214, 113, 18, 42, 104, 2, 99, 1, 128, 112, 128,
    181, 18, 138, 104, 254, 99, 10, 128, 112, 128, 213, 63, 1, 18, 162, 97,
    2, 128, 21, 63, 1, 18, 186, 128, 21, 63, 1, 18, 200, 128, 21, 63, 1, 18,
    194, 96, 32, 240, 24, 34, 212, 142, 52, 34, 212, 102, 62, 51, 1, 102, 3,
    104, 254, 51, 1, 104, 2, 18, 22, 121, 255, 73, 254, 105, 255, 18, 200,
    121, 1, 73, 2, 105, 1, 96, 4, 240, 24, 118, 1, 70, 64, 118, 254, 18,
    108, 162, 242, 254, 51, 242, 101, 241, 41, 100, 20, 101, 0, 212, 85,
    116, 21, 242, 41, 212, 85, 0, 238, 128, 128, 128, 128, 128, 128, 128, 0,
    0, 0, 0, 0 ];
    fn main2() {
        let mut b = chip8::CPU::new();
        b.load_rom(&INTRO);
        loop{
            b.run_loop();

        }
       
    }
fn main() {



    let tick_rate = Duration::from_millis(10);
    let mut b = chip8::CPU::new();
    b.load_rom(&INTRO);
    enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend) ;
    let mut last_tick = Instant::now();
    
    loop{

         let _ = terminal.as_mut().unwrap().draw(|x|
        {

            let canvas = Canvas::default().background_color(Color::Black)
            .block(Block::default().borders(Borders::ALL).title("Chip8"))
            .marker(Marker::Block)
            .paint(|ctx| {
             




                for x in -32..32{
 
                    for y in -16..16{

                        let color =b.display[2047-((y+16)*64+(31-x)) as usize];
                        if color == 1 
                        {

                            ctx.draw(
                            
                                &Rectangle {
                                    x: x as f64,
                                    y: y as f64,
                                    width: 1.0,
                                    height: 1.0,
                                    color: Color::Red,
                                }
                        
                        
                        );

                        }
                        else if 2047-((y+16)*64+(31-x)) < 512{
                            ctx.draw(
                            
                                &Rectangle {
                                    x: x as f64,
                                    y: y as f64,
                                    width: 1.0,
                                    height: 1.0,
                                    color: Color::Black,
                                }
                        
                        
                        );
                        }
                       


                        
                    }

                } 


             
                // for x in 0..64{
        
                //     canvas.set_pixel(x,y, Color::rgba(b.display[((y*32)+x) as usize], 0, 0, 255));
        
                // }


            })
            
            .x_bounds([-32.0,32.0])
            .y_bounds([-16.0,16.0]);
         x.render_widget(canvas,x.size());


         b.run_loop();
         
 
          
        });
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
           
            if event::poll(timeout).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                     match key.code {
                        KeyCode::Char('w') => {
                           b.keyboard[1]=1;
                           b.keyboard[4]=0;
                        //    b.display[0] = 0;
                           
                        }
                        KeyCode::Char('a') => {
                            b.keyboard[4]=0;  
                            b.keyboard[1]=0;                         
                        }
                        KeyCode::Char('s') => {
                            b.keyboard[4]=1;
                            b.keyboard[1]=0;
                        }
                        
                         
                        _ => {}
                    }
                }
            }
    }
    
    
}
