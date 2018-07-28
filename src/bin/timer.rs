/* ----------- */
/* -- Timer -- */
/* ----------- */

extern crate std;
extern crate libc;

use std::thread;
use std::sync::mpsc;
use std::error::Error;

pub enum TimerState {
    Start,
    Pause,
    End
}

pub struct Timer {
    pub thread: std::thread::JoinHandle<()>,
    pub tx: mpsc::Sender<TimerState>,
    pub rx: mpsc::Receiver<u64>,
}

impl Timer {
    pub fn new() -> Self {
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

        Self { thread, tx, rx }
    }

}
