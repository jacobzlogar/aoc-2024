use std::collections::BinaryHeap;
pub fn run() -> miette::Result<()> {
    crate::get_input("day9")?;
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct File {
    id: usize,
    file_size: usize,
    block_size: isize 
}

#[derive(Debug, Clone)]
struct FileSystem {
    files: Vec<File>
}

impl FileSystem {
    fn remaining_blocksize(&mut self) -> usize {
        self.files.iter().fold(0, |acc, e| acc + e.block_size as usize)
    }

    fn print_files(&mut self) {
        for file in self.files.clone() {
            for _ in 0..file.file_size {
                print!("{}", file.id)
            }
            for _ in 0..file.block_size {
                print!(".")
            }
        }
        println!("")
    }

    fn build(&mut self) -> Vec<usize> {
        self.files.iter().flat_map(|file| {
            return std::iter::repeat(file.id).take(file.file_size);
        }).collect::<Vec<usize>>()
    }
    
    fn parse(text: &str) -> Self {
        let mut files: Vec<File> = vec![];
        let mut id = 0;
        text.chars().enumerate().for_each(|(index, ch)| {
            let x = ch.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                files.push(File {
                    id,
                    file_size: x,
                    block_size: 0
                });
                id += 1;
            } else {
                files[id-1].block_size = x as isize;
            }
        });
        Self { files }
    }

    fn solve(&mut self) -> usize {
        let mut output: Vec<usize> = vec![];
        let mut files = self.files.clone();
        let mut block_map = self.build();
        let mut files_iter = files.iter();
        while let Some(mut file) = files_iter.next().cloned() {
            if file.block_size > 0 {
                output.extend(std::iter::repeat(file.id).take(file.file_size).collect::<Vec<usize>>());
            }
            while file.block_size > 0 {
                if let Some(last) = block_map.pop() {
                    output.push(last);
                    if let Some(mut f) = files.iter().find(|file| file.id == last).cloned() {
                        println!("{:?}", f);
                        f.block_size -= 1;
                    }
                }
                file.block_size -= 1;
            }
        }
        println!("{:?}", output);
        
        // println!("{:?}", output);
        1
    }
}

const TEST: &str = "2333133121414131402";
#[cfg(test)]
#[test]
fn test_day9_part1() {
    let mut fs = FileSystem::parse(TEST);
    fs.print_files();
    fs.solve();
}
