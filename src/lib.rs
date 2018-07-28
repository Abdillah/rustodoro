extern crate libc;

pub enum Message {
    SetInterval(u64),
    Start(u64),
    Reset,
    TriggerTime(u64),
    // DisplayTask(String),
    // DisplayStatus(String),
    Quit,
}

#[derive(Clone)]
pub struct Model {
    pub interval: u64,
    pub time_now: u64,
    pub time_start: Option<u64>,
    pub is_started: bool,
    pub is_quit: bool,
}

impl ::std::default::Default for Model {
    fn default() -> Self {
        Model { interval: 25, time_now: 0, time_start: None, is_started: false, is_quit: false }
    }
}

impl Model {
    pub fn update(self: Self, msg: Message) -> Self {
        match msg {
            Message::SetInterval(interv) => Model { interval: interv, ..self.clone() },
            Message::Start(time) => Model { time_start: Some(time), is_started: true, ..self.clone() },
            Message::Reset => Model { time_start: None, is_started: false, ..self.clone() },
            Message::TriggerTime(time) => {
                if self.time_start.is_some() {
                    if self.time_now > (self.time_start.unwrap() + self.interval) {
                        Model { time_now: time, time_start: None, is_started: false, ..self.clone() }
                    } else {
                        Model { time_now: time, ..self.clone() }
                    }
                } else { self }
            },
            Message::Quit => Model { is_quit: true, ..self.clone() },

            // Message::Now(time) => Model { time_now: time, ..self.clone() },
            // Message::Run(is_run) => Model { is_timer_run: is_run, ..self.clone() },
            // Message::DisplayStatus(s) => Model { message: s, ..self.clone() },
            // Message::Reset => Model { /* seconds: 25, */ ..self.clone() },
        }
    }

    pub fn update_many(self: Self, msgs: Vec<Message>) -> Self {
        let mut model = self;
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
