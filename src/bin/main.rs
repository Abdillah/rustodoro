extern crate ncurses;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::error::Error;

fn timer_start(start_time: u32) -> (std::thread::JoinHandle<()>, mpsc::Receiver<u32>) {
    let (tx, rx): (mpsc::Sender<u32>, mpsc::Receiver<u32>) = mpsc::channel();

    let tx_thread = tx.clone();
    let thread = thread::spawn(move || {
        for i in (0..start_time).rev() {
            for _ in 0..1001 {
                thread::sleep(Duration::from_millis(1));
            }
            tx_thread.send(i).unwrap();
        }
    });
    
    (thread, rx)
}

// fn destroy_win(win: ncurses::WINDOW)
// {
//     use ncurses::*;
//     const WINDOW_HEIGHT: i32 = 40;
//     const WINDOW_WIDTH: i32 = 40;
// 
//     let ch = ' ' as chtype;
//     wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
//     wrefresh(win);
//     delwin(win);
// }

fn gui_start() {
    use ncurses::*;
    
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();
    timeout(10);

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    printw("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Press F1 to exit");
    refresh();
}

fn typewriter_getfont<'a>(ch: char) -> Vec<Vec<char>> {
    let lineiter = match ch {
        ':' => r#"
....
.##.
....
.##.
....
        "#,
        '0' => r#"
######
#....#
#....#
#....#
######
        "#,
        '1' => r#"
.....#
.....#
.....#
.....#
.....#
        "#,
        '2' => r#"
######
.....#
######
#.....
######
        "#,
        '3' => r#"
######
.....#
######
.....#
######
        "#,
        '4' => r#"
#....#
#....#
######
.....#
.....#
        "#,
        '5' => r#"
######
#.....
######
.....#
######
        "#,
        '6' => r#"
######
#.....
######
#....#
######
        "#,
        '7' => r#"
######
.....#
....#.
...#..
...#..
        "#,
        '8' => r#"
######
#....#
######
#....#
######
        "#,
        '9' => r#"
######
#....#
######
.....#
######
        "#,
        _ => r#"
......
......
......
......
......
        "#,
    }.trim().split('\n')
    .map(|s| {
        let mut v = vec!();
        let string = s.to_owned().into_bytes();
        let chariter = string.iter().map(|c| *c as char);
        for ch in chariter {
            v.push(ch);
        }
        v
    });
    
    let mut lines = vec!();
    for line in lineiter {
        lines.push(line);
    }
    lines
}

struct Rectangle(i32, i32, i32, i32);

fn typewriter_printch(x: i32, y: i32, ch: char) -> Rectangle {
    let glyph = typewriter_getfont(ch);
    
    let mut rect = Rectangle(0, 0, 0, 0);
    
    for sel_glyph_row in 0..(glyph.len() as i32) {
        let glyphline = &glyph[sel_glyph_row as usize];
        
        for sel_glyph_col in 0..(glyphline.len() as i32) {
            if glyphline[sel_glyph_col as usize] == '#' {
                if ncurses::has_colors() == false {
                    ncurses::endwin();
                    panic!("Your terminal does not support color");
                }
                ncurses::start_color();
                ncurses::use_default_colors();
                ncurses::init_pair(1, -1, ncurses::constants::COLOR_RED);
    
                ncurses::attron(ncurses::COLOR_PAIR(1));
                ncurses::mvaddch(y + sel_glyph_row, x + sel_glyph_col, ' ' as u32);
                ncurses::attroff(ncurses::COLOR_PAIR(1));
            } else {
                // Delete
                ncurses::mvaddch(y + sel_glyph_row, x + sel_glyph_col, ' ' as u32);
            }
            
            rect = Rectangle(x, y, sel_glyph_col,  sel_glyph_row);
        }
    }
    
    rect
}

fn typewriter_print<'a>(x: i32, y: i32, s: &'a str) -> Rectangle {
    let xspacing = 1;
    let mut x = x;
    
    let mut textrect = Rectangle(x, y, 0, 0);
    
    for ch in s.chars() {
        let rect = typewriter_printch(x, y, ch);
        x += rect.2 + 1 + xspacing;
        textrect.2 += xspacing + 1 + rect.2;
    }
    
    textrect
}

fn main() {
    println!("Timer v0.1.0");
    
    // Timer start
    let start_time = 25;
    let (_, rx) = timer_start(start_time);
    
    gui_start();
    
    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);

    /* Start in the center. */
    let start_x = ncurses::COLS()/2i32 - 17;
    let start_y = ncurses::LINES()/2i32 - 3;
    let win = ncurses::newwin(7, 36, start_y - 1, start_x - 2);
    ncurses::box_(win, 0, 0);
    ncurses::wrefresh(win);

    let mut ch = ncurses::getch();
    while ch != ('q' as i32) {
        ch = ncurses::getch();
        if ch != -1 {
            ncurses::mvprintw(ncurses::LINES() - 1, 0, format!("You press {}", ch).as_str());
        }
        
        let time_left: Option<u32> = rx.try_recv()
        .map_err(|e| if e == mpsc::TryRecvError::Disconnected {
            panic!("{:?}", e.cause().unwrap())
        } else {
            e
        })
        .ok();
        if let Some(time) = time_left {
            ncurses::mvprintw(ncurses::LINES() - 1, 0, format!("Time left: {}s", time).as_str());
            typewriter_print(start_x, start_y, format!("{:02}:{:02}", (time / 60) as u32, time % 60).as_str());
        }
    }

    ncurses::endwin();
}