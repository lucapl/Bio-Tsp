mod utils;

use utils::randomizer::Randomizer;

fn main() {
    let mut rand = Randomizer::new();

    let max = 10;

    for _i in 0..3{
        let x = rand.random(max);
        let y = rand.different_random(x,max);
        println!("Random number {}, different random {}",x,y);

        if x==y{
            println!("!!! RANDOM GENERATOR DOES NOT WORK !!!")
        }
    }
    let random_perm = *rand.randomPermutation(10);
    println!("Random permutation {:?}",random_perm);
}
