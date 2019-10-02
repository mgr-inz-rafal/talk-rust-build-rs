use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern { fn hello(); }

fn precalc() -> Vec<u16> {
    let mut tab = Vec::new();
    for i in 0..359 {
        tab.push(((f64::from(i).to_radians().sin() + 1.0) * 50.0).round() as u16);
    }
    tab
}

fn main() -> Result<(), std::io::Error> {
    unsafe { hello(); }
    let tab = precalc();
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("beret.rs");
    let mut file = File::create(&path)?;
    file.write_all("const TAB: [u16; 359] = [".as_bytes())?;

    tab.iter().for_each(|v| {
        let _ = file.write_all((*v).to_string().as_bytes());
        let _ = file.write_all(",".as_bytes());
    });

    file.write_all("];".as_bytes())?;
    Ok(())
}
