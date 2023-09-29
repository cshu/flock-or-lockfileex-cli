#![allow(clippy::print_literal)]
#![allow(clippy::needless_return)]
#![allow(dropping_references)]
#![allow(clippy::assertions_on_constants)]
mod common;

use common::{CustRes, CustomErr};

use log::*;
use std::process::*;
use std::*;

#[macro_use(defer)]
extern crate scopeguard;

//(i) microsoft docs mentions: If a process terminates with a portion of a file locked or closes a file that has outstanding locks, the locks are unlocked by the operating system. However, the time it takes for the operating system to unlock these locks depends upon available system resources. Therefore, it is recommended that your process explicitly unlock all files it has locked when it terminates.
//(ii) I heard flock is similar. When process terminates file descriptors are closed so it is unlocked

fn main() -> ExitCode {
    env::set_var("RUST_BACKTRACE", "1"); //? not 100% sure this has 0 impact on performance? Maybe setting via command line instead of hardcoding is better?
                                         //env::set_var("RUST_LIB_BACKTRACE", "1");//? this line is useless?
                                         ////
    env::set_var("RUST_LOG", "trace"); //note this line must be above logger init.
    env_logger::init();

    let args: Vec<String> = env::args().collect(); //Note that std::env::args will panic if any argument contains invalid Unicode.
    fn the_end() {
        if std::thread::panicking() {
            info!("{}", "PANICKING");
        }
        info!("{}", "FINISHED");
    }
    defer! {
        the_end();
    }
    if main_inner(args).is_err() {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}
fn main_inner(args: Vec<String>) -> CustRes<()> {
    use fs2::*;
    let dur: u64 = args[2].parse()?;
    let filenm = &args[1];
    let filep = path::Path::new(filenm);
    if !filep.try_exists()? {
        fs::write(filep, b"\n")?;
    }
    info!("{}", "FILE EXISTENCE ENSURED");
    let fobj = fs::File::open(filep)?; //both windows and unix allow File::open simultaneously
    info!("{}", "FILE OPENED");
    fobj.lock_exclusive()?; //both windows and unix block here
    info!("{}", "LOCKED");
    fn the_unlock(the_fobj: fs::File) {
        if let Err(err) = the_fobj.unlock() {
            info!("{}", "FAILED TO UNLOCK");
            info!("{}", err);
        } else {
            info!("{}", "UNLOCKED");
        }
    }
    defer! {
        the_unlock(fobj);
    }
    thread::sleep(time::Duration::from_millis(dur));
    Ok(())
}
