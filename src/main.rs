use core::arch::x86_64::_rdrand64_step;
const  _ERROR : i8 = 0;
const  _PASS : i8 = 1;

fn main() -> () {

    let random_number: Result<u8, String> = gen_rand();
    match random_number {
        Ok(result) => {
            println!("✅Random number : {}", result);
        }
        Err(e) => {
            print!("{}", e);
        }
    }

    return ();
}

fn gen_rand() -> Result<u8, String> {
    let mut random : u64 = 0; // stored value
    unsafe {
        // bool
        let b_check: i8 = _rdrand64_step(&mut random) as i8;
        if b_check != _PASS {
            return Err("🚫Cannot generate random numbers".to_string());
        }
        return  Ok(random as u8);
    }

}