use nanite::keys::{Seed, ToPrivKey};

fn main() {
    let hex = "5cbc5b95dbce06c65106e2bbed72c221";
    let seed1 = Seed::new(hex);
    let hash1 = seed1.to_key(1);

    let seed2 = Seed::new(hex);
    let hash2 = seed2.to_key(1);
    println!("h1: {:X?}\nh2: {:X?}", &hash1, &hash2);
    assert_eq!(hash1, hash2);
}
