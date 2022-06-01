use anyhow::Result;
use std::io::empty;

// #![feature(test)]
// extern crate test;
mod stack_heap;

use std::error::Error;
use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, io, path};

fn main() {
    //     stack_heap::array_vector::run();
    //
    //     fn reverse_string(string: &str) {
    //         // let vect_str: Vec<char> = string.chars().collect();
    //         //
    //         // let mut reverse_str: Vec<char> = Vec::new();
    //         // for i in 0..string.len() {
    //         //     println!("{}", vect_str[i]);
    //         //     reverse_str.push([])
    //         // }
    //         // println!("{:?}", reverse_str);
    //         // for i in 0..string.len() {
    //         //     let pop_str = reverse_str.pop();
    //         //     println!("{:?}", pop_str);
    //         // }
    //     }
    //
    //     // reverse_string("hogehgoe")
    //
    //     // // makeeeさんの回答
    //     // pub fn check_kaibun(s: &str) -> bool {
    //     //     let chars: Vec<char> = s.chars().collect();
    //     //     for (i, c) in chars.iter().enumerate() {
    //     //         if chars[chars.len() - (i + 1)] != *c {
    //     //             return false;
    //     //         }
    //     //     }
    //     //     true
    //     // }
    //
    //     //     tree command by self create
    let entry = PathBuf::from("/Users/k-shiraishi/develop/solidjs-tutorial_220217");

    fn hoge(entry: PathBuf) -> anyhow::Result<()> {
        for path in read_dir(entry) {
            for path in path {
                let mut path_ref = path.unwrap().path();
                println!(
                    "> {:?}",
                    path_ref.clone().into_os_string().into_string().unwrap()
                );
                if path_ref.is_dir() {
                    for path_child in path_ref.read_dir()?.into_iter() {
                        hoge(path_child?.path())?
                        //
                    }
                }
            }
        }
        Ok(())
    }

    hoge(entry);
}

// #[cfg(test)]
// mod tests {
//     use crate::foo::fibo;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_fibo(b: &mut Bencher) {
//         b.iter(|| fibo());
//     }
// }
