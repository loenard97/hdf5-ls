use hdf5::File;
use clap::Parser;

use hdf5_ls::Printable;


#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    file_name: String,
    #[arg(short, long, default_value_t = false)]
    plain: bool,
    h5_objects: Option<Vec<String>>,
}


fn main() {
    let args = Args::parse();
    let file = File::open(args.file_name).expect("could not open file");

    if args.h5_objects.is_none() {
        if args.plain {
            file.recursive_print().expect("could not read file");
        } else {
            file.recursive_print_pretty(1).expect("could not read file");
        }
        return;
    }

    let mut iters = vec![];
    for object in args.h5_objects.unwrap() {
        let group = file.group(&object);
        if group.is_ok() {
            if args.plain {
                group.unwrap().recursive_print().expect("could not read file");
            } else {
                group.unwrap().recursive_print_pretty(1).expect("could not read file");
            }
            continue;
        }
        
        let dataset = file.dataset(&object);
        if dataset.is_ok() {
            let dataset = dataset.unwrap();

            if !args.plain {
                print!("{};", dataset.name());
            }

            iters.push(dataset.read_1d::<i64>().unwrap());
        }
    }

    if !args.plain {
        print!("\n");
    }
    if iters.len() > 0 {
        alternate_print(iters);
    }
}

fn alternate_print<T: std::fmt::Debug>(iterators: Vec<impl IntoIterator<Item = T>>) {
    let mut iterators = iterators.into_iter().map(IntoIterator::into_iter).collect::<Vec<_>>();
    let mut current = 0;

    loop {
        let iter = &mut iterators[current];
        match iter.next() {
            Some(item) => {
                print!("{:?}", item);
            }
            None => {
                iterators.remove(current);
                if iterators.is_empty() {
                    return;
                }
            }
        }

        if current % iterators.len() == iterators.len() - 1 {
            print!("\n");
        } else {
            print!(";");
        }
        current = (current + 1) % iterators.len();
    }
}
