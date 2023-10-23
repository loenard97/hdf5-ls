use hdf5::File;
use clap::Parser;

use hdf5_ls::Printable;


#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    file_name: String,
    #[arg(short, long, default_value_t = false)]
    plain: bool,
}


fn main() {
    let args = Args::parse();
    let file = File::open(args.file_name).expect("could not open file");

    if args.plain {
        file.recursive_print().expect("could not read file");
    } else {
        file.recursive_print_pretty(1).expect("could not read file");
    }
}
