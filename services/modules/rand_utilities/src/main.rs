#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use serde_json;
use tinyrand::{Rand, RandRange, Seeded, StdRand};

pub fn main() {}

#[marine]
#[derive(Debug)]
pub struct RandResult {
    pub stdout: String,
    pub stderr: String,
}

#[marine]
pub fn idx_from_range(lower: u32, upper: u32, seed: u64) -> RandResult {
    if lower >= upper {
        return RandResult {
            stdout: "".to_string(),
            stderr: format!("invalid range values. lower: {}, upper: {}", lower, upper),
        };
    }

    let mut rand = StdRand::seed(seed);
    let rnd_index = rand.next_range(lower..upper);
    RandResult {
        stdout: serde_json::json!({ "index": rnd_index }).to_string(),
        stderr: "".to_string(),
    }
}

#[marine]
pub fn item_from_vec(items: Vec<String>, seed: u64) -> RandResult {
    let mut rand = StdRand::seed(seed);
    let item_idx = rand.next_range(0..items.len());
    let item = items[item_idx].clone();
    RandResult {
        stdout: serde_json::json!({ "item": item }).to_string(),
        stderr: "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(
        config_path = "/Users/bebo/localdev/rand-utils/.fluence/tmp/Config.toml",
        modules_dir = "/Users/bebo/localdev/rand-utils/services/modules/rand_utilities/target/wasm32-wasi/release"
        // config_path = "../../../.fluence/tmp/Config.toml",
        // modules_dir = "../../services/modules/rand_utilities/target/wasm32-wasi/release"
    )]
    fn test_idx(rand_utils: marine_test_env::rand_utilities::ModuleInterface) {
        let seed = 123456789u64;
        let lower = 0u32;
        let upper = 5u32;
        let result = rand_utils.idx_from_range(lower.clone(), upper.clone(), seed.clone());
        let idx: serde_json::Value = serde_json::from_str(&result.stdout).unwrap();
        let idx = idx["index"].as_u64().unwrap() as u32;

        assert_eq!(result.stderr, "".to_string());
        assert!(idx <= upper);
        assert!(idx >= lower);
        // assert_eq!(idx, 1u32);

        let seed = 1234567891u64;
        let result = rand_utils.idx_from_range(lower.clone(), upper.clone(), seed.clone());
        let idx: serde_json::Value = serde_json::from_str(&result.stdout).unwrap();
        let idx = idx["index"].as_u64().unwrap() as u32;

        assert_eq!(result.stderr, "".to_string());
        assert!(idx <= upper);
        assert!(idx >= lower);
        // assert_eq!(idx, 2u32);

        let lower = 5u32;
        let upper = 5u32;
        let result = rand_utils.idx_from_range(lower.clone(), upper.clone(), seed);

        assert!(result.stderr.len() > 0);
        assert_eq!(
            result.stderr,
            format!("invalid range values. lower: {}, upper: {}", lower, upper)
        );
    }

    #[marine_test(
        config_path = "/Users/bebo/localdev/rand-utils/.fluence/tmp/Config.toml",
        modules_dir = "/Users/bebo/localdev/rand-utils/services/modules/rand_utilities/target/wasm32-wasi/release"
        // config_path = "../../../.fluence/tmp/Config.toml",
        // modules_dir = "../../services/modules/rand_utilities/target/wasm32-wasi/release"
    )]
    fn test_item(rand_utils: marine_test_env::rand_utilities::ModuleInterface) {
        let available_peers = vec![
            "12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e".to_owned(),
            "12D3KooWR4cv1a8tv7pps4HH6wePNaK6gf1Hww5wcCMzeWxyNw51".to_owned(),
            "12D3KooWKnEqMfYo9zvfHmqTLpLdiHXPe4SVqUWcWHDJdFGrSmcA".to_owned(),
            "12D3KooWHLxVhUQyAuZe6AHMB29P7wkvTNMn7eDMcsqimJYLKREf".to_owned(),
            "12D3KooWJd3HaMJ1rpLY1kQvcjRPEvnDwcXrH8mJvk7ypcZXqXGE".to_owned(),
        ];
        let seed = 123456789u64;
        let result = rand_utils.item_from_vec(available_peers.clone(), seed.clone());

        assert!(result.stderr.len() == 0);

        let peer_id: serde_json::Value = serde_json::from_str(&result.stdout).unwrap();
        let peer_id: String = peer_id["item"].to_string();

        // println!("peer id: {}", peer_id);
        // println!("peer id: {}", available_peers.contains(&peer_id));
        // println!("{:?}", available_peers);
        // wtf??
        assert!(available_peers.contains(&peer_id));
        // see https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c14d386ca0e2f31c1f0879de2f6ff78e
    }
}
