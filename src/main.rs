use core::arch::x86_64::_rdrand64_step;
const  _ERROR : i8 = 0;
const  _PASS : i8 = 1;

fn main() -> () {
    
    let _check_arch = check_arch();

    let random_number: Result<u64, String> = gen_rand();
    match random_number {
        Ok(result) => {
            println!("✅: {}", result);
        }
        Err(e) => {
            print!("{}", e);
        }
    }

    return ();
}

/// [Intel's documentation]
/// (<https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand64_step>)

pub fn gen_rand() -> Result<u64, String> {
    let mut random : u64 = 0;
    unsafe {
        let b_check: i32 = _rdrand64_step(&mut random);
        if b_check != _PASS as i32 {
            return Err("🚫Failed to generate a random number".to_string());
        }
        return  Ok(random);
    }

}

pub fn check_arch() -> Result<(), String> {
    if cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") {
        return Ok(());
    }
    else {
        return Err(String::from("🚫Error: This program requires Intel x86 or x86_64 architecture."));
    }
}