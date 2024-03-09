mod utils;

use utils::randomizer::Randomizer;

fn main() {
    let mut rand = Randomizer::new();

    let max = 10;

    for i in 0..1000{
        let x = rand.random(max);
        let y = rand.different_random(x,max);
        println!("Random number {}, different random {}",x,y);

        if x==y{
            println!("!!! RANDOM GENERATOR DOES NOT WORK !!!")
        }
    }
}
