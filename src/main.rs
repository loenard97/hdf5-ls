use std::{env, fs};
use hdf5::{File, Group, Dataset, Result};
use std::error::Error;
use csv::Writer;

pub trait Printable {
    fn recursive_print(&self, indent: usize) -> Result<()>;
    fn to_csv(&self, path: &String) -> Result<(), Box<dyn Error>>;
}

impl Printable for File {
    fn recursive_print(&self, indent: usize) -> Result<()> {
        println!(" +- File {:?}", self.name());
        for val in self.datasets()? {
            val.recursive_print(indent)?;
        }
        for val in self.groups()? {
            val.recursive_print(indent)?;
        }
        Ok(())
    }

    fn to_csv(&self, path: &String) -> Result<(), Box<dyn Error>> {
        for val in self.datasets()? {
            val.to_csv(path)?;
        }
        for val in self.groups()? {
            val.to_csv(path)?;
        }
        Ok(())
    }
}

impl Printable for Group {
    fn recursive_print(&self, indent: usize) -> Result<()> {
        let outstr = "   ".repeat(indent);
        println!("{} +- Group {:?}", outstr, self.name());
        for val in self.datasets()? {
            val.recursive_print(indent+1)?;
        }
        for val in self.groups()? {
            val.recursive_print(indent+1)?;
        }
        Ok(())
    }

    fn to_csv(&self, path: &String) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(path.to_owned() + &self.name())?;
        for val in self.datasets()? {
            val.to_csv(path)?;
        }
        for val in self.groups()? {
            val.to_csv(path)?;
        }
        Ok(())
    }
}

impl Printable for Dataset {
    fn recursive_print(&self, indent: usize) -> Result<()> {
        let outstr = "   ".repeat(indent);
        println!("{} +- Dataset {:?}", outstr, self.name());
        Ok(())
    }

    fn to_csv(&self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = Writer::from_path(path.to_owned() + &self.name())?;
        println!("{}D data type {:?}", self.ndim(), self.dtype());
        let vector: Vec<f32> = self.read_raw().unwrap();
        let data_str = vector.iter().map(|x| x.to_string() + ", ").collect::<String>();
        println!("{}", data_str);
        file.write_record(&[data_str])?;
        file.flush()?;
        Ok(())
    }
}

fn read_file(file_name: String) -> Result<()> {
    let file = File::open(file_name)?;
    file.recursive_print(1)?;
    Ok(())
}

fn print_file_test() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    match read_file(file_name.to_string()) {
        Ok(_) => return,
        Err(_) => println!("Could not read File."),
    };
}

fn to_csv(file_name: String) {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {println!("Could not read File."); return},
    };
    match file.to_csv(&"out".to_string()) {
        Ok(_) => return,
        Err(e) => println!("Could not read File: {}", e),
    };
}

fn to_csv_test() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    to_csv(file_name.to_string());
}

fn main() {
    println!("Print File:");
    print_file_test();

    println!("\nPrint to CSV:");
    to_csv_test();
}