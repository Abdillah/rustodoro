extern crate core;
#[macro_use]
extern crate neon;
extern crate rustodoro;

use neon::prelude::*;

#[no_mangle]
pub extern fn __cxa_pure_virtual() {
    loop{};
}

/* -------- STORE ----------- */

#[derive(Clone)]
pub struct Store {
    pub state: rustodoro::Model,
}

impl Store {
    fn new() -> Self {
        Self { state: rustodoro::Model::default() }
    }

    fn dispatch(&mut self, action: rustodoro::Message) {
         self.state = self.state.update(action);
    }
}

declare_types! {
    pub class JsStore for Store {
        init(_cx) {
            Ok(Store::new())
        }

        method dispatch(mut cx) {
            let action = cx.argument::<JsObject>(0)?;
            let action_type = action.get(&mut cx, "type")?
            .downcast::<JsString>().or_throw(&mut cx)?
            .value().to_uppercase();

            let mut this = cx.this();
            {
                let msg = {
                    match action_type.as_str() {
                        "SET_INTERVAL" => rustodoro::Message::SetInterval(action.get(&mut cx, "interval")?.downcast::<JsNumber>().or_throw(&mut cx)?.value()),
                        "START" => rustodoro::Message::Start(action.get(&mut cx, "startTime")?.downcast::<JsNumber>().or_throw(&mut cx)?.value()),
                        "TIMER_RESET" => rustodoro::Message::Reset,
                        "TIMER_TRIGGER" => rustodoro::Message::TriggerTime(action.get(&mut cx, "triggerTime")?.downcast::<JsNumber>().or_throw(&mut cx)?.value()),
                        "QUIT" => rustodoro::Message::Quit,
                        _ => rustodoro::Message::Iddle,
                    }
                };
                let guard = cx.lock();
                let mut store = this.borrow_mut(&guard);
                store.dispatch(msg);
            }

            Ok(action.upcast())
        }

        method getState(mut cx) {
            let state = JsObject::new(&mut cx);
            let this = cx.this();
            let (interval, time_now, time_start, is_started, is_quit) = {
                let guard = cx.lock();
                let store = this.borrow(&guard);

                (store.state.interval, store.state.time_now, store.state.time_start, store.state.is_started, store.state.is_quit)
            };

            let interval = cx.number(interval).upcast::<JsValue>();
            let time_now = cx.number(time_now).upcast::<JsValue>();
            let time_start = match time_start {
                Some(value) => cx.number(value).upcast::<JsValue>(),
                None => cx.null().upcast::<JsValue>(),
            };
            let is_started = cx.boolean(is_started).upcast::<JsValue>();
            let is_quit = cx.boolean(is_quit).upcast::<JsValue>();

            state.set(&mut cx, "interval", interval).expect("Set failed");
            state.set(&mut cx, "timeNow", time_now).expect("Set failed");
            state.set(&mut cx, "timeStart", time_start).expect("Set failed");
            state.set(&mut cx, "isStarted", is_started).expect("Set failed");
            state.set(&mut cx, "isQuit", is_quit).expect("Set failed");

            Ok(state.upcast())
        }

        method get(mut cx) {
            let attr: String = cx.argument::<JsString>(0)?.value();

            let this = cx.this();

            match &attr[..] {
                "interval" => {
                    let interval = {
                        let guard = cx.lock();
                        let store = this.borrow(&guard);
                        store.state.interval
                    };
                    Ok(cx.number(interval).upcast())
                },
                "timeNow" => {
                    let time_now = {
                        let guard = cx.lock();
                        let store = this.borrow(&guard);
                        store.state.time_now
                    };
                    Ok(cx.number(time_now).upcast())
                },
                "timeStart" => {
                    let time_start = {
                        let guard = cx.lock();
                        let store = this.borrow(&guard);
                        store.state.time_start
                    };

                    Ok(match time_start {
                        Some(time_start) => cx.number(time_start).upcast(),
                        None => cx.null().upcast(),
                    })
                },
                "isStarted" => {
                    let is_started = {
                        let guard = cx.lock();
                        let store = this.borrow(&guard);
                        store.state.is_started
                    };
                    Ok(cx.boolean(is_started).upcast())
                },
                "isQuit" => {
                    let is_quit = {
                        let guard = cx.lock();
                        let store = this.borrow(&guard);
                        store.state.is_quit
                    };
                    Ok(cx.boolean(is_quit).upcast())
                },
                _ => cx.throw_type_error("property does not exist")
            }
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsStore>("Store")
});
