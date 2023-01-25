use std::{io::{Seek, BufReader, BufRead}, fs::File, collections::BTreeSet};


crate::main!(7);


fn main_0(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut fs = Directory::new("/".to_owned());
    let mut nav = DirectoryNavigator::new(&mut fs);
    for line in input.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "$" => {
                match words.next().unwrap() {
                    "cd" => {
                        let name = words.next().unwrap();
                        nav.chdir(match name {
                            "/" => continue,
                            ".." => None,
                            subdir_name => Some(subdir_name)
                        }).expect("Nonexistant subdir");
                    },
                    "ls" => {},
                    _ => panic!("Invalid command"),
                }
            },
            "dir" => {
                let name = words.next().unwrap();
                nav.get_dir().create_subdir(name.to_string()).expect("Duplicate subdirectories");
            },
            int => {
                let name = words.next().unwrap();
                let size = int.parse().unwrap();
                nav.get_dir().add_file(name.to_owned(), size).expect("Duplicate files");
            }
        };
    }
    let mut nav = DirectoryNavigator::new(&mut fs);

    let mut sum_size = 0;
    while let Ok(()) = nav.next_dir() {
        if nav.get_dir().total_size() <= 100000 {
            sum_size += nav.get_dir().total_size();
        }
    }
    println!("part 1: {sum_size}");
    Ok(())
}


fn main_1(input: &mut BufReader<File>) -> Result<(), std::io::Error> {
    let mut fs = Directory::new("/".to_owned());
    let mut nav = DirectoryNavigator::new(&mut fs);
    for line in input.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next().unwrap() {
            "$" => {
                match words.next().unwrap() {
                    "cd" => {
                        let name = words.next().unwrap();
                        nav.chdir(match name {
                            "/" => continue,
                            ".." => None,
                            subdir_name => Some(subdir_name)
                        }).expect("Nonexistant subdir");
                    },
                    "ls" => {},
                    _ => panic!("Invalid command"),
                }
            },
            "dir" => {
                let name = words.next().unwrap();
                nav.get_dir().create_subdir(name.to_string()).expect("Duplicate subdirectories");
            },
            int => {
                let name = words.next().unwrap();
                let size = int.parse().unwrap();
                nav.get_dir().add_file(name.to_owned(), size).expect("Duplicate files");
            }
        };
    }

    let mut smallest = fs.total_size();
    let needed = smallest - 40_000_000;
    let mut nav = DirectoryNavigator::new(&mut fs);
    while let Ok(()) = nav.next_dir() {
        let curr_size = nav.get_dir().total_size();
        if curr_size > needed && smallest > curr_size {
            smallest = curr_size;
        }
    }
    println!("part 2: {smallest}");
    Ok(())
}


#[derive(Debug)]
struct DirectoryError;


#[derive(Debug)]
struct Directory {
    name: String,
    files: BTreeSet<String>,
    local_size: i32,
    subdirs: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {name, files: BTreeSet::new(), local_size: 0, subdirs: Vec::new()}
    }

    fn add_file(&mut self, str: String, size: i32) -> Result<(), DirectoryError> {
        if self.files.contains(&str) {
            return Err(DirectoryError);
        }
        self.local_size += size;
        self.files.insert(str);
        Ok(())
    }
    
    fn create_subdir(&mut self, name: String) -> Result<(), DirectoryError> {
        if self.subdirs.iter().any(|dir| dir.name == name) {
            return Err(DirectoryError);
        }
        self.subdirs.push(Directory::new(name));
        Ok(())
    }

    fn total_size(&self) -> i32 {
        let mut size = self.local_size;
        for subdir in &self.subdirs {
            size += subdir.total_size();
        }
        size
    }

    fn get_subdir(&mut self, indices: &[usize]) -> &mut Directory {
        if indices.is_empty() {
            return self;
        }
        self.subdirs[indices[0]].get_subdir(&indices[1..])
    }
}


#[derive(Debug)]
struct DirectoryNavigator<'a> {
    fs: &'a mut Directory,
    location: Vec<usize>,
}


impl<'a> DirectoryNavigator<'a> {
    fn new(fs: &'a mut Directory) -> Self {
        Self {fs, location: Vec::new()}
    }

    fn chdir(&mut self, subdir_name: Option<&str>) -> Result<(), DirectoryError> {
        if let Some(name) = subdir_name {
            for (index, dir) in self.get_dir().subdirs.iter().enumerate() {
                if dir.name == name {
                    self.location.push(index);
                    return Ok(());
                }
            }
            return Err(DirectoryError)
        } else {
            self.location.pop();
        }
        Ok(())
    }

    fn get_dir(&mut self) -> &mut Directory {
        self.fs.get_subdir(&self.location)
    }

    fn next_dir(&mut self) -> Result<(), DirectoryError> {
        if !self.get_dir().subdirs.is_empty() {
            self.location.push(0);
            return Ok(());
        }
        while let Some(mut last) = self.location.pop() {
            last += 1;
            if self.get_dir().subdirs.len() != last {
                self.location.push(last);
                return Ok(());
            }
        }
        Err(DirectoryError)
    }
}
