/* ----------- */
/* -- Timer -- */
/* ----------- */

extern crate std;
extern crate libc;

use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::ops::Drop;

pub fn get_current_timestamp() -> f64 {
    let spec = unsafe {
        // libc::timespec { tv_sec: 0, tv_nsec: 0 };
        let mut spec = std::mem::uninitialized();
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut spec);
        spec
    };

    spec.tv_sec as f64
}

enum TimerState {
    Start,
    Pause,
    End
}

pub struct Timer {
    thread: Option<std::thread::JoinHandle<()>>,
    tx: mpsc::Sender<TimerState>,
    rx: mpsc::Receiver<f64>,
}

/// `struct Timer` internally implement separate thread with mpsc duplex communication. The thread
/// will be cleaned up once the `struct Timer` goes out of scope.
impl Timer {
    pub fn new() -> Self {
        let (tx_thread, rx): (mpsc::Sender<f64>, mpsc::Receiver<f64>) = mpsc::channel();
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

                if let Err(e) = tx_thread.send(get_current_timestamp()) {
                    panic!("Timer thread error: {}", e);
                };
            }
        });

        Self { thread: Some(thread), tx, rx }
    }

    pub fn start(&self) {
        let _ = self.tx.send(TimerState::Start);
    }

    pub fn stop(&self) {
        let _ = self.tx.send(TimerState::Pause);
    }

    pub fn get_time(&self) -> Option<f64> {
        self.rx.try_recv()
        .map_err(|e| if e == mpsc::TryRecvError::Disconnected {
            panic!("{:?}", e.cause().unwrap())
        } else { e })
        .ok()
    }
}


impl Drop for Timer {
    fn drop(&mut self) {
        let _ = self.tx.send(TimerState::End);

        if let Some(thread) = self.thread.take() {
            thread.join().expect("failed to join thread");
        }
    }
}
