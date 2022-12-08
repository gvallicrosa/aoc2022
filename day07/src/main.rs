use std::collections::VecDeque;

struct File {
    // name: String,
    size: usize,
}
struct Directory {
    name: String,
    folders: Vec<Directory>,
    files: Vec<File>,
    size: Option<usize>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.into(),
            folders: Vec::new(),
            files: Vec::new(),
            size: None,
        }
    }

    fn compute_size(&mut self) -> usize {
        if self.size.is_none() {
            let sfiles: usize = self.files.iter().map(|f| f.size).sum();
            let sfolders: usize = self.folders.iter_mut().map(|d| d.compute_size()).sum();
            self.size = Some(sfiles + sfolders);
            // println!("folder '{}': {}", self.name, self.size.unwrap());
        }
        self.size.unwrap()
    }

    fn get_subfolder(&mut self, name: &str) -> Option<&mut Directory> {
        self.folders.iter_mut().find(|folder| folder.name == name)
    }

    fn add_folder(&mut self, mut path: VecDeque<String>) {
        if !path.is_empty() {
            let folder = path.pop_front().unwrap();
            let subfolder = match self.get_subfolder(&folder) {
                Some(f) => f,
                None => {
                    self.folders.push(Directory::new(&folder));
                    self.folders.last_mut().unwrap()
                }
            };
            subfolder.add_folder(path);
        }
    }

    fn add_file(&mut self, mut path: VecDeque<String>, file: File) {
        if !path.is_empty() {
            // move to folder
            let folder = path.pop_front().unwrap();
            let subfolder = self.get_subfolder(&folder).unwrap();
            subfolder.add_file(path, file);
        } else {
            // add file
            self.files.push(file);
        }
    }

    fn get_below_value(&self, threshold: &usize) -> usize {
        let mut sum = 0;
        if &self.size.unwrap() < threshold {
            sum += self.size.unwrap()
        }
        for folder in self.folders.iter() {
            sum += folder.get_below_value(threshold);
        }
        sum
    }

    fn get_min_to_delete(&self, threshold: &usize) -> Option<usize> {
        let min_others = self
            .folders
            .iter()
            .flat_map(|f| f.get_min_to_delete(threshold))
            .min();
        if let Some(v) = min_others {
            Some(v)
        } else if &self.size.unwrap() >= threshold {
            Some(self.size.unwrap())
        } else {
            None
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut path: VecDeque<String> = VecDeque::new();
    let mut root = Directory::new("");
    let lines: Vec<_> = include_str!("input.txt").split('\n').collect();
    // let lines: Vec<_> = include_str!("input_example.txt").split('\n').collect();
    for line in lines {
        // get line
        let temp: Vec<_> = line.split(' ').collect();
        // command
        if temp[0].starts_with('$') {
            if temp[1].starts_with("cd") {
                if temp[2].starts_with('/') {
                } else if temp[2].starts_with("..") {
                    path.pop_back();
                } else {
                    path.push_back(temp[2].into());
                    root.add_folder(path.clone());
                }
            } else if temp[1].starts_with("ls") {
            } else {
                unreachable!();
            }
        }
        // file
        else if temp[0] != "dir" {
            // dbg!(&temp);
            let size = temp[0].parse::<usize>().unwrap();
            // let name: String = temp[1].into();
            let file = File { size };
            // let file = File { size, name };
            root.add_file(path.clone(), file);
        }
    }
    // dive to sizes
    root.compute_size();
    dbg!(root.get_below_value(&100_000));

    // second
    let total_disk_space = 70_000_000;
    let needed_unused_space = 30_000_000;
    let current_free = total_disk_space - root.size.unwrap();
    let minimum_to_free = needed_unused_space - current_free;
    dbg!(current_free);
    dbg!(minimum_to_free);
    dbg!(root.get_min_to_delete(&minimum_to_free).unwrap());

    Ok(())
}
