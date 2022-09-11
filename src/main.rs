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

    if let Some(mask) = args.mask {
        let bits_numbers: Vec<u32> = mask
            .split(",")
            .map(|x| x.parse::<u32>().expect("Error in parsing mask u32s"))
            .collect();
        let mut bitvec1 = vec![1; bits_numbers[0] as usize];
        let mut bitvec0 = vec![0; bits_numbers[1] as usize];
        bitvec1.append(&mut bitvec0);
        println!("{:?}", bitvec1);
    }
}
