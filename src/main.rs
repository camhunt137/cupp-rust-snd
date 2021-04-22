use dashmap::DashMap;
use rand::Rng;
use std::ops::Add;
use std::sync::Arc;

fn gen_library(n: usize) -> DashMap<u8,f32> {
    let mut rng = rand::thread_rng();
    let map = DashMap::with_capacity(n);

    for _ in 0..n {
        let r: u8 = rng.gen();
        let f: f32 = rng.gen();
        map.entry(r).or_insert(f);
    }

    return  map
}

struct SeqIterator {
    ln: usize,
    mx: usize,
    n: usize,
}

impl SeqIterator {
    pub fn new(ln: usize, mx: usize) -> SeqIterator {
        let seq_iterator = SeqIterator {
            ln,
            mx,
            n: 0
        };
        return seq_iterator
    }
}

impl Iterator for SeqIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        self.n += 1;
        if self.n > self.mx {
            return  None;
        }

        let mut rng = rand::thread_rng();
        let vals: Vec<u8> = (0..self.ln).map(|_| rng.gen()).collect();
        return Some(vals);

    }
}

struct ValIterator<'a>  {
    v: &'a Vec<u8>,
    n: usize,
    mx: usize,
    pos: usize,
    l: usize,
}

impl ValIterator<'_> {
    pub fn new(v: &Vec<u8>, mx: usize) -> ValIterator {
        let val_iterator = ValIterator {
            v,
            mx,
            n: 0,
            pos: 0,
            l: v.len()
        };
        return val_iterator
    }
}

impl Iterator for ValIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {

        if self.n > self.mx {
            self.n = 0;
            self.pos += 1;
            if self.pos == self.l {
                return None
            }
        }

        let cur: u8 = self.v[self.pos];
        let n: u8 = self.n as u8;

        self.n += 1;

        return Some(cur.wrapping_add(n));
    }
}


fn main() {

    // create random library with int as keys
    let map = gen_library(1000000);

    // iterator
    let seq_iterator = SeqIterator::new(10000, 1000);


    // single thread implementation
    // // get all the seqs
    // for seq in seq_iterator {
    //     let mut seq_score :f32 = 0.0;
    //     let val_iterator = ValIterator::new(&seq,8);
    //     for val in val_iterator {
    //         if map.contains_key(&val) {
    //             let f : f32 = *map.get(&val).unwrap();
    //             seq_score = seq_score.add(f);
    //         }
    //     }
    //     println!("Score: {}",seq_score);
    // }

    // multiple thread I think, with thread pool
    let pool = threadpool::Builder::new()
        .num_threads(12)
        .build();

    // // counted
    let map = Arc::new(map);

    for seq in seq_iterator {
        let map = map.clone();
        pool.execute(move|| {
            let mut seq_score: f32 = 0.0;
            let val_iterator = ValIterator::new(&seq, 8);
            for val in val_iterator {
                if map.contains_key(&val) {
                    let f: f32 = *map.get(&val).unwrap();
                    seq_score = seq_score.add(f);
                }
            }
            println!("Score: {}", seq_score);
        })
    }

    // wait to finish
    pool.join();

}
