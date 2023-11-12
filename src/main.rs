// use rsfbclient::{prelude::*, FbError, Row, DynLink, Connection};
use rsfbclient::FbError;

mod db_routines;
mod fb_base;

// use db_routines::tasks::insert_task;
// use db_routines::tasks::insert_task_with_params;
// use db_routines::tasks::print_tasks_as_tuples;
// use db_routines::tasks::print_tasks_as_rows;
use db_routines::tasks::{print_last_task, delete_task};

// fn main() {
//     println!("Hello, world!");
// }

fn main() -> Result<(), FbError> {
    // let _ = insert_task();
    // let _ = insert_task_with_params();
    // let _ = print_tasks_as_tuples();
    // let _ = print_tasks_as_rows();
    let _ = delete_task(104);
    let _ = print_last_task();
    Ok(())
}