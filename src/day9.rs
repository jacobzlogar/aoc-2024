use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;

pub fn run() -> miette::Result<()> {
    let data = crate::get_input("day9")?;
    println!("Day 9, part 1: {}", part_1(&data));
    Ok(())
}

fn part_1(data: &str) -> usize {
    let mut fs = FileSystem::parse(&data);
    fs.solve(false)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
struct File {
    id: usize,
    file_size: usize,
    block_size: isize,
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.block_size.cmp(&other.block_size)
    }
}

#[derive(Debug, Clone)]
struct FileSystem {
    heap: BinaryHeap<File>,
    files: Vec<File>,
}

impl FileSystem {
    // fn remaining_blocksize(&mut self) -> usize {
    //     self.files.iter().fold(0, |acc, e| acc + e.block_size as usize)
    // }

    #[tracing::instrument(skip_all)]
    fn solve(&mut self, contiguous: bool) -> usize {
        let mut output = 0;
        let mut id = 0;
        for i in 0..self.files.len() {
            let mut file = self.files[i];
            for _ in 0..file.file_size {
                output += file.id * id;
                id += 1;
            }
            file.file_size = 0;
            if let Some(mut last) = self.heap.pop() {}
        }
        // let mut output: Vec<usize> = vec![];
        // let mut file_map = self.build();
        // let mut files = self.files.clone();
        // for i in 0..files.len() {
        //     let mut file = files[i];
        //     if file.file_size == 0 {
        //         break;
        //     }
        //     output.extend(std::iter::repeat(file.id).take(file.file_size).collect::<Vec<usize>>());
        //     if let Some(last) = file_map.pop() {
        //         if let Some(existing) = files.iter_mut().find(|f| f.id == last) {
        //             existing.file_size = 0;
        //             existing.block_size = 0;
        //         }
        //         output.push(last);
        //     }
        //     file.block_size = 0;
        // }
        // output.iter().enumerate().fold(0, |acc, (id, value)| acc + (id * value))
        output
    }

    fn print_files(&mut self) {
        // for file in self.files.clone() {
        //     for _ in 0..file.file_size {
        //         print!("{}", file.id)
        //     }
        //     for _ in 0..file.block_size {
        //         print!(".")
        //     }
        // }
        println!("")
    }

    fn build(&mut self) -> Vec<usize> {
        // self.files.iter().flat_map(|file| {
        //     return std::iter::repeat(file.id).take(file.file_size);
        // }).collect::<Vec<usize>>()
        vec![]
    }

    #[tracing::instrument(skip_all)]
    fn parse(text: &str) -> Self {
        let mut heap = BinaryHeap::new();
        let mut files = vec![];
        let mut char_iter = text.chars().enumerate().peekable();
        let mut id = 0;
        while let Some((_, file_size)) = char_iter.next() {
            let file_size = file_size.to_digit(10).unwrap() as usize;
            if let Some((_, block_size)) = char_iter.next() {
                if let Some(block_size) = block_size.to_digit(10) {
                    let file = File {
                        id,
                        block_size: block_size as isize,
                        file_size,
                    };
                    id += 1;
                    heap.push(file);
                    files.push(file)
                }
            } else {
                let file = File {
                    id,
                    file_size,
                    block_size: 0,
                };
                heap.push(file);
                files.push(file)
            }
        }
        Self { files, heap }
    }
}

const TEST: &str = "12345";

#[cfg(test)]
#[test]
fn test_day9_part1() {
    println!("{:?}", part_1(TEST));
    //assert_eq!(part_1(TEST), 1928);
}
