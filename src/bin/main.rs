extern crate libc;
extern crate ncurses;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::error::Error;

/* ----------- */
/* -- Timer -- */
/* ----------- */
fn timer_tick() -> (std::thread::JoinHandle<()>, mpsc::Sender<u64>,  mpsc::Receiver<u64>)
{
    let (tx, rx): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();

    let tx_thread = tx.clone();
    let thread = thread::spawn(move || {
        loop {
            let mut spec;
            unsafe {
                // libc::timespec { tv_sec: 0, tv_nsec: 0 };
                spec = std::mem::uninitialized();
                libc::clock_gettime(libc::CLOCK_REALTIME, &mut spec);
            }

            tx_thread.send(spec.tv_sec as u64).unwrap();
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

/* ------------------------- */
/* --- Model and Message --- */
/* ------------------------- */
enum Message {
    Run(bool),
    Decrement,
    Reset,
    DisplayStatus(String),
    Quit,
}

#[derive(Clone)]
struct Model {
    pub is_timer_run: bool,
    pub seconds: u32,
    pub is_end: bool,
    pub message: String,
}

impl std::default::Default for Model {
    fn default() -> Self { Model { is_timer_run: false, seconds: 25, is_end: false, message: String::from("Press q to exit") } }
}

impl Model {
    fn update(self: &Self, msg: Message) -> Self {
        match msg {
            Message::Decrement => Model { seconds: self.seconds - 1, ..self.clone() },
            Message::Run(is_run) => Model { is_timer_run: is_run, ..self.clone() },
            Message::DisplayStatus(s) => Model { message: s, ..self.clone() },
            Message::Reset => Model { seconds: 25, ..self.clone() },
            Message::Quit => Model { is_end: true, ..self.clone() },
        }
    }

    fn update_many(self: &Self, msgs: Vec<Message>) -> Self {
        let mut model = self.clone();
        for msg in msgs {
            model = model.update(msg);
        }
        model
    }
}


/* ------------ */
/* --- View --- */
/* ------------ */
#[derive(Clone)]
struct RenderInfo {
    pub timer_pos_x: i32,
    pub timer_pos_y: i32,
    pub time: String,
    pub message: String,
}

impl std::default::Default for RenderInfo {
    fn default() -> Self {
        RenderInfo {
            timer_pos_x: 0,
            timer_pos_y: 0,
            time: String::new(),
            message: String::new(),
        }
    }
}

impl std::cmp::PartialEq for RenderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.timer_pos_x == other.timer_pos_x
        && self.timer_pos_y == other.timer_pos_y
        && self.time == other.time
        && self.message == other.message
    }
}

impl RenderInfo {
    fn from_model(model: &Model) -> Self {
        let seconds = model.seconds;
        let time_fmted = format!("{:02}:{:02}", (seconds / 60) as u32, seconds % 60);

        let start_x = ncurses::COLS()/2i32 - 17;
        let start_y = ncurses::LINES()/2i32 - 3;

        Self {
            timer_pos_x: start_x - 1,
            timer_pos_y: start_y - 2,
            time: time_fmted,
            message: model.message.clone(),
        }
    }

    /* This function may induce side-effect */
    fn render(self, model: &Model) -> Result<Self, String> {
        // Start in the center
        let win = ncurses::newwin(7, 36, self.timer_pos_y - 1, self.timer_pos_x - 2);
        ncurses::box_(win, 0, 0);
        ncurses::wrefresh(win);

        ncurses::mvprintw(ncurses::LINES() - 1, 0, "                                                ");
        ncurses::mvprintw(ncurses::LINES() - 1, 0, self.message.as_str());

        typewriter_print(self.timer_pos_x, self.timer_pos_y, self.time.clone().as_str());

        Ok(self)
    }
}

/* ------------ */
/* --- Init --- */
/* ------------ */
fn main() {
    println!("Timer v0.1.0");

    // Init model
    let mut model = Model::default();

    // Init RenderInfo
    let mut rendinfo = RenderInfo::from_model(&model);
    rendinfo = rendinfo.render(&model).unwrap();

    // Init GUI
    gui_start();

    // Spawn timer
    let (_, _, rx) = timer_tick();

    loop {
        // Query for keypress
        let ch = ncurses::getch();
        model = model.update(Message::DisplayStatus(format!("{}", ch)));

        if ch == ('q' as i32) {
            // Send Quit Message!
            model = model.update_many(vec!(
                Message::DisplayStatus(String::from("Quit signal sent..")),
                Message::Quit,
            ));
        }

        if ch == ('r' as i32) {
            model = model.update_many(vec!(
                Message::DisplayStatus(String::from("Reset!")),
                Message::Reset,
            ));
        }

        if ch == ('s' as i32) {
            model = model.update_many(vec!(
                Message::DisplayStatus(String::from("Start!")),
                Message::Run(!model.is_timer_run),
            ));
        }

        // Query for time ticks
        let timetick: Option<u64> = rx.try_recv()
        .map_err(|e| if e == mpsc::TryRecvError::Disconnected {
            panic!("{:?}", e.cause().unwrap())
        } else { e })
        .ok();



        if model.is_timer_run {
            if let Some(nowtick) = timetick {
                // Send Decrement Message!
                model = model.update(Message::Decrement);

                let status = format!("Time ticking.. {}", model.seconds);
                model = model.update(Message::DisplayStatus(status));
            }
        }

        // Update for GUI
        rendinfo = rendinfo.render(&model).unwrap();
        if model.is_end {
            break;
        }
    }

    gui_end();
}
