#[cfg(test)]
mod tests;
mod utils;

extern crate nix;

use std::ffi::CString;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{close, execvp, fork, ForkResult};

// Executed command: less access.log | grep -E "(WARN|ERROR|FATAL)" | awk "{print($2)}" | uniq -c | sort -nrk1

fn main() {
    let args = tty2cstr![less access.log | grep -E "(WARN|ERROR|FATAL)" | awk "{print $2}" | uniq -c | sort -nrk1];
    println!("{:?}", args);

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let args = tty2cstr![less access.log];
            println!("{:?}", args);

            close(1).unwrap();
            open(
                "errors.log",
                OFlag::O_CREAT | OFlag::O_WRONLY,
                Mode::from_bits_truncate(0o777),
            )
            .unwrap();

            execvp(&CString::new("less").unwrap(), args.as_slice()).unwrap();
        }

        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }

        Err(err) => {
            panic!("Failed to execute fork: {}", err);
        }
    };
}
