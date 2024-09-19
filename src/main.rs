use core::arch::x86_64::_rdrand64_step;
const  _ERROR : i8 = 0;
const  _PASS : i8 = 1;

/// [Intel's documentation]
/// (<https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand64_step>)
fn main() -> () {
    
    let _check_arch = check_arch();

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

/// L'instruction RDRAND fait partie de la famille d'instructions Intel Secure Key, 
/// qui exploite un générateur de nombres aléatoires matériel basé sur des phénomènes physiques pour produire de l'aléatoire
/// 
pub fn gen_rand() -> Result<u8, String> {
    let mut random : u64 = 0; // stored value
    unsafe {
        let b_check: i8 = _rdrand64_step(&mut random) as i8;
        if b_check != _PASS {
            return Err("🚫Cannot generate random numbers".to_string());
        }
        return  Ok(random as u8);
    }

}

pub fn check_arch() -> Result<(), String> {
    if cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") {
        return Ok(());
    }
    else {
        return Err(String::from("🚫Requirements intel Arch"));
    }
}