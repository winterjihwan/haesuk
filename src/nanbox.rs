use core::f64;

const NAN_MASK_BITS: u64 = 1u64 << 12 - 1;

pub fn nanbox() {
    let one: f64 = 1.0;
    let two: f64 = 2.0;
    let two_hundred: f64 = 200.0;
    let inf: f64 = 1.0 / 0.0;
    let neg_inf: f64 = -1.0 / 0.0;
    let nan: f64 = f64::NAN;
    inspect_f64(one, "one");
    inspect_f64(two, "two");
    inspect_f64(two_hundred, "two_hundred");
    inspect_f64(inf, "inf");
    inspect_f64(neg_inf, "neg_inf");
    inspect_f64(nan, "nan");
}

fn is_nan(f: f64) -> bool {
    f.to_bits() & NAN_MASK_BITS == NAN_MASK_BITS
}

fn inspect_f64(f: f64, label: &str) {
    let bits = format!("{:064b}", f.to_bits());
    let bits_chunked = bits
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}:", label);
    println!("\t{bits_chunked}");
    println!("\tStd is_nan: {is_nan}", is_nan = f.is_nan());
    println!("\tSelf is_nan: {is_nan}\n", is_nan = is_nan(f));
}
