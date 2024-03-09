use rand::prelude::*;

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

}



