// cargo-deps: rand = "0.8.5"
extern crate rand;

use rand::Rng;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut rng = rand::thread_rng();

    let vector_of_umbers: Vec<u8> = (0..10).map(|_| rng.gen_range(0..=100)).collect();

    // let mut vector_of_umbers = vec![1, 2, 3];
    // for _ in 0..100 {
    //     vector_of_umbers.push(rng.gen_range(0..=100));
    // }

    println!("{:?}", vector_of_umbers);

    println!("Average = {}", average(&vector_of_umbers));

    println!("Median = {:?}", median(&vector_of_umbers));

    println!("Mode = {:?}", mode(&vector_of_umbers));
}

fn average(numbers: &[u8]) -> f32 {
    let length: usize = numbers.len();

    numbers.iter().fold(0.0, |result, &value| {
        result + (value as f32) / (length as f32)
    })
}

fn median(numbers: &[u8]) -> Option<f32> {
    let copy = {
        let mut v = numbers.to_vec();
        v.sort();
        v
    };

    let length = numbers.len();
    let half_of_length = numbers.len() / 2;

    match length {
        0 => None,
        _ if length % 2 == 0 => {
            Some((copy[half_of_length] + copy[half_of_length - 1]) as f32 / 2.0)
        }
        _ => Some(copy[half_of_length] as f32),
    }
}

fn mode(numbers: &[u8]) -> Option<u8> {
    numbers
        .iter()
        .fold(
            (None, 0, &mut HashMap::<u8, u8>::new()),
            |(winner, winner_count, map): (Option<u8>, u8, &mut HashMap<u8, u8>), &value| {
                let current_count = *map.entry(value).and_modify(|c| *c += 1).or_insert(1);

                match winner_count >= current_count {
                    true => (winner, winner_count, map),
                    false => (Some(value), current_count, map),
                }
            },
        )
        .0
}
