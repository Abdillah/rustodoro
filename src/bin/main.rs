extern crate libc;
extern crate ncurses;
extern crate rustodoro;

use std::thread;
use std::sync::mpsc;
use std::error::Error;

use ::rustodoro::Message;
use ::rustodoro::Model;

/* ----------- */
/* -- Timer -- */
/* ----------- */
pub enum TimerState {
    Start,
    Pause,
    End
}

fn timer_tick() -> (std::thread::JoinHandle<()>, mpsc::Sender<TimerState>, mpsc::Receiver<u64>)
{
    let (tx_thread, rx): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    let (tx, rx_thread): (mpsc::Sender<TimerState>, mpsc::Receiver<TimerState>) = mpsc::channel();

    let tx_thread = tx_thread.clone();

    let thread = thread::spawn(move || {
        let mut state = TimerState::Pause;
        loop {
            // let thread_cmd: Option<TimerState> = rx_thread.try_recv()
            let thread_cmd: Option<TimerState> = rx_thread.recv_timeout(std::time::Duration::from_millis(500))
            .map_err(|e| if e == mpsc::RecvTimeoutError::Disconnected {
                panic!("{:?}", e.cause().unwrap())
            } else { e })
            .ok();
            if let Some(cmd) = thread_cmd {
                state = cmd;
            };

            match state {
                TimerState::Pause => continue,
                TimerState::End => break,
                _ => (),
            };

            let spec = unsafe {
                // libc::timespec { tv_sec: 0, tv_nsec: 0 };
                let mut spec = std::mem::uninitialized();
                libc::clock_gettime(libc::CLOCK_REALTIME, &mut spec);
                spec
            };

            if let Err(e) = tx_thread.send(spec.tv_sec as u64) {
                panic!("Timer thread error: {}", e);
            };
        }
    });

    (thread, tx, rx)
}

/* --------- */
/* -- GUI -- */
/* --------- */
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
    // printw("Use the arrow keys to move");
    // mvprintw(LINES() - 1, 0, "Press q to exit");
    refresh();
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
                ncurses::initscr();
                if !ncurses::has_colors() {
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

fn gui_end() {
    use ncurses::*;

    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    echo();
    clear();
    endwin();
}

/* ------------ */
/* --- View --- */
/* ------------ */
use std::collections::HashMap;

#[derive(Clone)]
struct RenderState {
    pub current_state: HashMap<String, String>,
    pub dirty_keys: Vec<String>,
}

impl std::default::Default for RenderState {
    fn default() -> Self {
        RenderState {
            current_state: HashMap::new(),
            dirty_keys: vec!(),
        }
    }
}

impl RenderState {
    pub fn from_model(model: &Model) -> Self {
        Self {
            current_state: Self::state_from_model(model),
            dirty_keys: vec!(),
        }
    }

    fn state_from_model(updated_model: &Model) -> HashMap<String, String> {
        let elapsed = if updated_model.is_started {
            updated_model.time_now - if let Some(time) = updated_model.time_start { time } else { 0 }
        } else { 0 };
        let seconds = updated_model.interval - elapsed;
        let time_fmted = format!("{:02}:{:02}", (seconds / 60) as u32, seconds % 60);

        let start_x = ncurses::COLS()/2i32 - 17;
        let start_y = ncurses::LINES()/2i32 - 3;

        let mut rstate: HashMap<String, String> = HashMap::new();
        rstate.insert("timer_pos_x".to_string(), (start_x - 1).to_string());
        rstate.insert("timer_pos_y".to_string(), (start_y - 2).to_string());
        rstate.insert("time".to_string(), time_fmted);
        rstate
    }

    pub fn diff_state(&self, model: &Model) -> Self {
        let mut rstate = Self::from_model(model);

        let dirty_keys: Vec<String> = Vec::new();
        for (key, val) in &self.current_state {
            if val.clone() != rstate.current_state[key] {
                rstate.dirty_keys.push(key.clone());
            }
        }

        rstate
    }
}

/* This function may induce side-effect */
fn render(rstate: RenderState, model: &Model) -> Result<RenderState, String> {
    let mut newrstate = rstate.diff_state(model);
    let state = newrstate.current_state;

    // Start in the center
    if newrstate.dirty_keys.contains(&"timer_pos_x".to_string()) || newrstate.dirty_keys.contains(&"timer_pos_y".to_string()) {
        let win = ncurses::newwin(7, 36, state["timer_pos_y"].parse::<i32>().unwrap() - 1, state["timer_pos_x"].parse::<i32>().unwrap() - 2);
        ncurses::box_(win, 0, 0);
        ncurses::wrefresh(win);

        typewriter_print(state["timer_pos_x"].parse::<i32>().unwrap(), state["timer_pos_y"].parse::<i32>().unwrap(), state["time"].clone().as_str());
    } else if newrstate.dirty_keys.contains(&"time".to_string()) {
        typewriter_print(state["timer_pos_x"].parse::<i32>().unwrap(), state["timer_pos_y"].parse::<i32>().unwrap(), state["time"].clone().as_str());
    }

    newrstate.current_state = state;
    Ok(newrstate)
}

/* ------------ */
/* --- Init --- */
/* ------------ */
fn main() {
    println!("Timer v0.1.0");

    // Init model
    let mut model = Model::default();

    // Init RenderState
    let mut rstate = RenderState::from_model(&model);
    rstate = render(rstate, &model).unwrap();

    // Init GUI
    gui_start();

    // Spawn timer
    let (thread, tx, rx) = timer_tick();

    loop {
        // Query for keypress
        let ch = match std::char::from_u32(ncurses::getch() as u32) {
            Some(ch) => ch,
            None => '\0'
        };

        model = if model.is_started {
            // Query for time ticks
            let timetick: Option<u64> = rx.try_recv()
            .map_err(|e| if e == mpsc::TryRecvError::Disconnected {
                panic!("{:?}", e.cause().unwrap())
            } else { e })
            .ok();

            let model = if let Some(nowtick) = timetick {
                // Send Decrement Message!
                model.update(Message::TriggerTime(nowtick))
            } else { model };
            model
        } else { model };

        model = match ch {
            'q' => model.update(Message::Quit),
            'r' => model.update(Message::Reset),
            's' => {
                let spec = unsafe {
                    let mut spec = std::mem::uninitialized();
                    libc::clock_gettime(libc::CLOCK_REALTIME, &mut spec);
                    spec
                };

                model = model.update(Message::Start(spec.tv_sec as u64));
                model.update(Message::TriggerTime(spec.tv_sec as u64))
            },
            _ => model
        };

        // -----------------
        // Side-effect part
        // -----------------

        // Update for GUI
        rstate = render(rstate, &model).unwrap();
        if model.is_started {
            tx.send(TimerState::Start);
        };

        if model.is_quit {
            tx.send(TimerState::End);
            break;
        };
    }

    let _ = thread.join();

    gui_end();
}
