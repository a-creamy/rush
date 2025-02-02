mod custom;
pub mod dir;
mod utils;

pub fn execute(cmd: &str) {
    let args = utils::tokenize(cmd);
    if args.is_empty() || custom::execute(&args) {
        return;
    }

    if utils::command_exists(&args[0]) {
        let args_slice: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
        let _ = utils::run_program(&args[0], &args_slice);
    } else {
        eprintln!("rush: {}: command not found", &args[0]);
    }
}
