#![allow(dead_code)]
pub mod java_rand;
pub mod chunk;
pub mod slime;
pub use java_rand::Rng;
pub use chunk::Chunk;
pub use slime::is_slime_chunk;
pub use slime::seed_from_slime_chunks;


use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_chunks_from_file(path: &str) -> Result<Vec<Chunk>, std::io::Error> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    read_chunks(&s)
}

// Format: each line is one chunk, comma separated: "x, y"
pub fn read_chunks(s: &str) -> Result<Vec<Chunk>, std::io::Error> {
    let mut v = vec![];
    let reader = s;

    for (_, line) in reader.lines().enumerate() {
        let mut n = line.split(',');
        // FIXME: rewrite this with proper error handling
        let x;
        let z;
        if let Some(s) = n.next() {
            if let Ok(i) = s.parse() {
                x = i;
            } else {
                continue;
            }
        } else {
            continue;
            //return Err(i)
        }
        if let Some(s) = n.next() {
            if let Ok(i) = s.parse() {
                z = i;
            } else {
                continue;
            }
        } else {
            continue;
            //return Err(i)
        }
        v.push(Chunk::new(x,z));
    }

    // Remove duplicates
    v.sort();
    v.dedup();

    Ok(v)
}

pub fn find_seed(chunks: &str, no_chunks: &str) -> Vec<u64> {
    let c = read_chunks(chunks);
    println!("{:#?}", c);
    let nc = read_chunks(no_chunks);
    println!("{:#?}", nc);

    if let (Ok(c), Ok(nc)) = (c, nc) {
        if (c.len() == 0) && (nc.len() == 0) {
            println!("Can't find seed without chunks");
            return vec![];
        } 
        let seeds = seed_from_slime_chunks(&c, 0, &nc, 0);
        println!("Found seeds:\n{:#?}", seeds);

        {
            // Display only seeds that could be generated by java (empty box)
            let java_seeds: Vec<_> = seeds
                .iter()
                .map(|&s| Rng::extend_long_48(s))
                .collect();

            println!("Java seeds: \n{:#?}", java_seeds);
        }

        seeds
    } else {
        vec![]
    }
}

pub fn generate_slime_chunks(seed: i64, limit: usize) -> Vec<Chunk> {
    generate_slime_chunks_or_not(true, seed, limit)
}

pub fn generate_no_slime_chunks(seed: i64, limit: usize) -> Vec<Chunk> {
    generate_slime_chunks_or_not(false, seed, limit)
}

pub fn generate_slime_chunks_or_not(slime: bool, seed: i64, limit: usize) -> Vec<Chunk> {
    let mut v = Vec::with_capacity(limit);
    for x in 0.. { // yeah just go on forever
        for z in -99..100 {
            let c = Chunk::new(x, z);
            if is_slime_chunk(seed as u64, &c) ^ (!slime) {
                v.push(c);
                if v.len() >= limit {
                    return v;
                }
            }
        }
    }

    // unreachable
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
