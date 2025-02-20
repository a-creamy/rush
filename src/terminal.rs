use std::sync::Mutex;

pub struct Terminal {
    pub print: String,
}

pub static TERMINAL: Mutex<Terminal> = Mutex::new(Terminal {
    print: String::new(),
});
