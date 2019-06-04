extern crate libc;

pub mod timer;

pub enum Message {
    SetInterval(f64),
    Start(f64),
    Reset,
    TriggerTime(f64),
    // DisplayTask(String),
    // DisplayStatus(String),
    Quit,
    Iddle,
}

#[derive(Clone)]
pub struct Model {
    pub interval: f64,
    pub time_now: f64,
    pub time_start: Option<f64>,
    pub is_started: bool,
    pub is_quit: bool,
}

impl ::std::default::Default for Model {
    fn default() -> Self {
        Model { interval: 25.0, time_now: 0.0, time_start: None, is_started: false, is_quit: false }
    }
}

impl Model {
    pub fn update(&self, msg: Message) -> Self {
        match msg {
            Message::SetInterval(interv) => Model { interval: interv, ..self.clone() },
            Message::Start(time) => Model { time_start: Some(time), is_started: true, ..self.clone() },
            Message::Reset => Model { time_start: None, is_started: false, ..self.clone() },
            Message::TriggerTime(time) => {
                if self.time_start.is_some() {
                    if self.time_now == (self.time_start.unwrap() + self.interval) {
                        Model { time_now: time, time_start: None, is_started: false, ..self.clone() }
                    } else {
                        Model { time_now: time, ..self.clone() }
                    }
                } else { self.clone() }
            },
            Message::Quit => Model { is_quit: true, ..self.clone() },
            Message::Iddle => Model { ..self.clone() },

            // Message::Now(time) => Model { time_now: time, ..self.clone() },
            // Message::Run(is_run) => Model { is_timer_run: is_run, ..self.clone() },
            // Message::DisplayStatus(s) => Model { message: s, ..self.clone() },
            // Message::Reset => Model { /* seconds: 25, */ ..self.clone() },
        }
    }

    pub fn update_many(&self, msgs: Vec<Message>) -> Self {
        let mut model = self.clone();
        for msg in msgs {
            model = model.update(msg);
        }
        model
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
