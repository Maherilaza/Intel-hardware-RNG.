use core::arch::x86_64::_rdrand64_step;

use colored::{ColoredString, Colorize};
const PASS : i8 = 1;

fn main() -> () {
    
    let _check_arch = check_arch();

    let random_number: Result<u64, String> = gen_rand();
    match random_number {
        Ok(result) => {
            println!("{} {}", "[*]".yellow() , result);
        }
        Err(e) => {
            print!("{}", e);
        }
    }

    return ();
}

/// [Intel's documentation]
/// (<https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand64_step>)

fn gen_rand() -> Result<u64, String> {
    let mut random : u64 = 0;
    unsafe {
        let b_check: i32 = _rdrand64_step(&mut random);
        if b_check != PASS as i32 {
            return Err(format!("{}", "[-] Cannot generate a random number".red()));
        }
        return  Ok(random);
    }
}

fn check_arch() -> Result<(), ColoredString> {
    if cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") {
        return Ok(());
    }
    else {
        return Err(format!("{}", "Error: This program requires Intel x86 or x86_64 architecture").red()
        );
    }
}