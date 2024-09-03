use core::f64;

const NAN_MASK_BITS: u64 = ((1u64 << 12) - 1) << 51;
const INF_MASK_BITS: u64 = ((1u64 << 11) - 1) << 52;
const TYPE_MASK_BITS: u64 = ((1u64 << 3) - 1) << 48;
const VALUE_MASK_BITS: u64 = (1u64 << 48) - 1;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum NanType {
    IntType = 0,
    PointerType = 1,
    FloatType,
}

//pub fn nanbox() {
//    let start = f64::NAN;
//    inspect_f64(start, "start");
//
//    let next = set_type(start, NanType::PointerType);
//    inspect_f64(next, "next");
//
//    assert_eq!(extract_type(next), NanType::PointerType);
//
//    let next = set_value(next, 1);
//    inspect_f64(next, "next");
//
//    assert_eq!(extract_value(next), 1);
//}

fn extract_type(f: f64) -> NanType {
    if !is_nan(f) {
        return NanType::FloatType;
    }

    let n = f.to_bits() & TYPE_MASK_BITS;
    let nan_type = (n >> 48) as u8;
    match nan_type {
        0u8 => NanType::IntType,
        1u8 => NanType::PointerType,
        _ => panic!(),
    }
}

fn extract_value(f: f64) -> u64 {
    let n = f.to_bits() & VALUE_MASK_BITS;
    n as u64
}

fn set_type(f: f64, nan_type: NanType) -> f64 {
    let f_bits = f.to_bits();
    let n: u64 = (nan_type as u64) << 48;
    f64::from_bits((f_bits & !TYPE_MASK_BITS) | n)
}

fn set_value(f: f64, value: u64) -> f64 {
    let f_bits = f.to_bits();
    let n: u64 = value as u64;
    f64::from_bits((f_bits & !VALUE_MASK_BITS) | n)
}

fn is_inf(f: f64) -> bool {
    !is_nan(f) && f.to_bits() & INF_MASK_BITS == INF_MASK_BITS
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
    println!("\tStd is_inf: {is_inf}", is_inf = f.is_infinite());
    println!("\tSelf is_inf: {is_inf}\n", is_inf = is_inf(f));
}
