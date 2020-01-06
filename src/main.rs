use rand::random;
use std::collections::HashMap;

fn random_permutation<T: Clone>(v: &Vec<T>) -> Vec<T> {
    let mut candidates = v.to_vec();
    let mut result: Vec<T> = Vec::new();
    while !candidates.is_empty() {
        let index: usize = random::<usize>() % candidates.len();
        let value: T = candidates.remove(index);
        result.push(value);
    }
    result
}

fn main() {
    let mut node_map: HashMap<String, Vec<String>> = HashMap::new();

    // Generate the nodes.
    let colors = ["red", "green", "blue", "yellow", "cyan"];
    for color in colors.iter() {
        node_map.insert(color.to_string(), Vec::new());
        for i in 0..20 {
            let node_name = format!("{}-{}", color, i);
            node_map
                .get_mut(&color.to_string())
                .unwrap()
                .push(node_name);
        }
    }

    // Generate the time series
    println!("time,left,right");
    for t in 0..100 {
        for color in colors.iter() {
            let nodes = node_map.get(&color.to_string()).unwrap();
            let permutation = random_permutation(&nodes);
            for (a, b) in nodes.iter().zip(&permutation) {
                println!("{},{},{}", t, a, b);
            }
        }
    }
}
