use nanite::keys::{Seed, ToPrivKey};

fn main() {
    let seed = Seed::new();
    let hash = seed.to_key(1);
    println!("{:?}", hash);
}
