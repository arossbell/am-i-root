// Am I Root?
// Adam R Bell (github.com/arossbell; https://keybase.io/iscsi); released under MIT License.
// Spawn a root shell if a root setuid is on this binary.
// Don't be stupid with this code.

use users::{get_effective_uid, get_current_uid, get_user_by_uid};
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;

fn main() {
    let uid = get_current_uid();
    let euid = get_effective_uid();
    if uid == 0 && euid == 0 {
        println!("Yes.");
    } else if uid != 0 && euid != 0 && uid == euid {
        println!("No.");
    } else if uid != 0 && euid == 0 {
        println!("Now you are.");
        Command::new("/bin/bash").uid(euid).stdin(Stdio::inherit()).stdout(Stdio::inherit()).output().unwrap();
    } else if uid != 0 && euid != 0 && uid != euid {
        println!("You aren't root, but now you're {}.", get_user_by_uid(euid).unwrap().name().to_string_lossy());
        Command::new("/bin/bash").uid(euid).stdin(Stdio::inherit()).stdout(Stdio::inherit()).output().unwrap();
    } else {
        panic!("An unexpected UID/EUID combination happened!");
    }
}
