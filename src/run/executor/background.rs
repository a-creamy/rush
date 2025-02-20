use crate::{run::{executor, node::Ast, ShellError}, terminal::TERMINAL};
use std::{collections::VecDeque, sync::Mutex, thread};

static JOB_ID_POOL: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
static JOB_ID_COUNTER: Mutex<u32> = Mutex::new(0);
static JOBS: Mutex<u32> = Mutex::new(0);

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    match lhs {
        Ast::Command(args) => {
            let job_id = {
                let mut pool = JOB_ID_POOL.lock().unwrap();
                if let Some(id) = pool.pop_front() {
                    id
                } else {
                    let mut counter = JOB_ID_COUNTER.lock().unwrap();
                    *counter += 1;
                    *counter
                }
            };

            let args = args.clone();

            thread::spawn(move || {
                {
                    let mut jobs = JOBS.lock().unwrap();
                    *jobs += 1;
                }
                println!("[{}] {}", job_id, args.join(" "));
                let mut terminal = TERMINAL.lock().unwrap();
                if let Err(e) = executor::execute(&Ast::Command(args.clone())) {
                    terminal.print = format!("[{}]+ {}\n", job_id, e);
                } else {
                    terminal.print = format!("[{}]+ Done\n", job_id);
                }
                {
                    let mut jobs = JOBS.lock().unwrap();
                    *jobs -= 1;
                    JOB_ID_POOL.lock().unwrap().push_back(job_id);
                }
            });
        }
        Ast::Background(left, right) => {
            execute(left, right)?;
        }
        ast => {
            executor::execute(ast)?;
        }
    }

    match rhs {
        Ast::Command(args) => {
            executor::execute(&Ast::Command(args.clone()))?;
        }
        Ast::Background(left, right) => {
            execute(left, right)?;
        }
        ast => {
            executor::execute(ast)?;
        }
    }

    Ok(())
}
