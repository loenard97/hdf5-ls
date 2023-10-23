use hdf5::{File, Group, Dataset, Result};

pub trait Printable {
    fn recursive_print(&self) -> Result<()>;
    fn recursive_print_pretty(&self, indent: usize) -> Result<()>;
}

impl Printable for File {
    fn recursive_print(&self) -> Result<()> {
        for dataset in self.datasets()? {
            dataset.recursive_print()?;
        }
        for group in self.groups()? {
            group.recursive_print()?;
        }
        Ok(())
    }

    fn recursive_print_pretty(&self, indent: usize) -> Result<()> {
        println!(" +- File {:?}", self.name());
        for val in self.datasets()? {
            val.recursive_print_pretty(indent)?;
        }
        for val in self.groups()? {
            val.recursive_print_pretty(indent)?;
        }
        Ok(())
    }
}

impl Printable for Group {
    fn recursive_print(&self) -> Result<()> {
        println!("{}", self.name());
        for dataset in self.datasets()? {
            dataset.recursive_print()?;
        }
        for group in self.groups()? {
            group.recursive_print()?;
        }
        Ok(())
    }

    fn recursive_print_pretty(&self, indent: usize) -> Result<()> {
        let outstr = "   ".repeat(indent);
        println!("{} +- Group {:?}", outstr, self.name());
        for val in self.datasets()? {
            val.recursive_print_pretty(indent+1)?;
        }
        for val in self.groups()? {
            val.recursive_print_pretty(indent+1)?;
        }
        Ok(())
    }
}

impl Printable for Dataset {
    fn recursive_print(&self) -> Result<()> {
        println!("{}", self.name());
        Ok(())
    }

    fn recursive_print_pretty(&self, indent: usize) -> Result<()> {
        let outstr = "   ".repeat(indent);
        println!("{} +- Dataset {:?}", outstr, self.name());
        Ok(())
    }
}
