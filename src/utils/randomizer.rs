use rand::prelude::*;
use std::vec::Vec;
use std::boxed::Box;

pub struct Randomizer{
    rng: ThreadRng,
}

impl Randomizer{
    pub fn new() -> Self {
        Randomizer{
            rng: rand::thread_rng(),
        }
    }

    pub fn random(&mut self,_max:u32)->u32{
        return self.rng.gen_range(0..=_max-1);
    }

    pub fn different_random(&mut self,x:u32,_max:u32)->u32{
        return (self.random(_max-1)+1+x)%_max
    }

    pub fn shuffle(&mut self, v: &mut Vec<i32>){
        let n = v.len() as u32;
        for i in 0..n{
            let r = self.random(n-i)+i;
            v.swap(i as usize,r as usize);
        }
    }

    pub fn randomPermutation(&mut self,length:i32)->Box<Vec<i32>>{
        let mut permutation:Vec<i32> = (0..length).collect();
        self.shuffle(&mut permutation);
        Box::new(permutation)
    }

}



