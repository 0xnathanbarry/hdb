use clap::Parser;
use hex;

/// A CLI app to convert hex decimal binary and create hex masks (converts hex to binary by default)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Hex conversions
    /// Convert Hex to Binary
    #[clap(long, value_parser)]
    hb: Option<String>,

    /// Convert Hex to Decimal
    #[clap(long, value_parser)]
    hd: Option<String>,

    // Binary conversions
    /// Convert Binary to Hex
    #[clap(long, value_parser)]
    bh: Option<String>,

    /// Convert Binary to Decimal
    #[clap(long, value_parser)]
    bd: Option<String>,

    // Decimal Conversions
    /// Convert Decimal to Hex
    #[clap(long, value_parser)]
    dh: Option<String>,

    /// Convert Decimal to Binary
    #[clap(long, value_parser)]
    db: Option<String>,

    /// Creates a binary mask. <Number of 1s>,<Number of 0s>
    #[clap(long, value_parser)]
    mask: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Hex to Binary
    if let Some(num) = args.hb {
        print!("Binary: ");
        hex::decode(num)
            .expect("Error when decoding Hex")
            .into_iter()
            .for_each(|x| {
                let mut former = format!("{:0>8b}", x);
                let latter = former.split_off(4);
                print!("{} {} ", former, latter);
            });
        println!("");
    }

    // Hex to Decimal
    if let Some(num) = args.hd {
        print!("Decimal: ");
        let mut s = String::new();
        hex::decode(num)
            .expect("Error when decoding Hex")
            .into_iter()
            .for_each(|x| {
                s.push_str(&format!("{:b}", x));
            });
        print!("{}", usize::from_str_radix(&s, 2).unwrap());
        println!("");
    }

    // Decimal to Binary
    if let Some(num) = args.db {
        println!("Binary: {:b}", num.parse::<usize>().unwrap());
    }

    // Get Mask
    if let Some(mask) = args.mask {
        let bits_numbers: Vec<usize> = mask
            .split(",")
            .map(|x| x.parse::<usize>().expect("Error in parsing mask u32s"))
            .collect();
        let mut bitvec1 = vec![1 as u8; bits_numbers[0]];
        let mut bitvec0 = vec![0 as u8; bits_numbers[1]];
        bitvec1.append(&mut bitvec0);
        let mut s = String::new();
        bitvec1
            .into_iter()
            .for_each(|x| s.push(if x == 1 { '1' } else { '0' }));
        let num = usize::from_str_radix(&s, 2).unwrap();
        println!("Binary Mask: {}", s);
        println!("Decimal: {}", num);
        println!("Hex: {:X}", num);
    }
}
