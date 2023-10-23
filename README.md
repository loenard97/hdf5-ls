<div align="center">

# hdf5-ls
A CLI tool to print data from an [HDF5](https://www.hdfgroup.org/solutions/hdf5/) file

![](https://img.shields.io/github/last-commit/loenard97/hdf5-ls?&style=for-the-badge&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/hdf5-ls?&style=for-the-badge&color=F74C00)

</div>


## 🖥️ Usage
The default options print the names of all groups and datasets of an h5 file recursively to stdout:
```txt
$ hdf5-ls my_file.h5
 +- File "/"
    +- Group "/My Group"
       +- Dataset "/My Group/First Dataset"
       +- Dataset "/My Group/Second Dataset"
    +- Group "/Another Group"
       +- Dataset "/Another Group/Third Dataset"
```

Pretty printing can be disabled with the `--plain` flag:
```txt
$ hdf5-ls --plain my_file.h5
/My Group
/My Group/First Dataset
/My Group/Second Dataset
/Another Group
/Another Group/Third Dataset
```


## 🔧 Installation
Download and install with [git](https://git-scm.com/docs/git) and [cargo](https://doc.rust-lang.org/book/ch01-01-installation.html):
```sh
git clone https://github.com/loenard97/hdf5-ls.git
cd hdf5-ls
cargo install --path .
```
