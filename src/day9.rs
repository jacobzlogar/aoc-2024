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

    #[tracing::instrument(skip_all)]
    fn solve(&mut self, contiguous: bool) -> usize {
        let mut output: Vec<usize> = vec![];
        let mut file_map = self.build();
        let mut files = self.files.clone();
        for i in 0..files.len() {
            let mut file = files[i];
            if file.file_size == 0 {
                break;
            }
            output.extend(std::iter::repeat(file.id).take(file.file_size).collect::<Vec<usize>>());
            if let Some(last) = file_map.pop() {
                if let Some(existing) = files.iter_mut().find(|f| f.id == last) {
                    existing.file_size = 0;
                    existing.block_size = 0;
                }
                output.push(last);
            }
            file.block_size = 0;
        }
        output.iter().enumerate().fold(0, |acc, (id, value)| acc + (id * value))
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
    
    #[tracing::instrument(skip_all)]
    fn parse(text: &str) -> Self {
        let mut files: Vec<File> = vec![];
        let mut id = 0;
        text.chars().enumerate().for_each(|(index, ch)| {
            if let Some(digit) = ch.to_digit(10) {
                if index % 2 == 0 {
                    files.push(File {
                        id,
                        file_size: digit as usize,
                        block_size: 0
                    });
                    id += 1;
                } else {
                    files[id-1].block_size = digit as isize;
                }
            }
        });
        Self { files }
    }
}

const TEST: &str = "2333133121414131402";

#[cfg(test)]
#[test]
fn test_day9_part1() {
    assert_eq!(part_1(TEST), 1928);
}
