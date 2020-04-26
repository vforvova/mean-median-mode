fn main() {
    let vec = vec![2, 1, 3, 39, 34, 19, 19];
    println!("Vector: {:?}", vec);
    println!("Mean: {:?}", mean(&vec));
    println!("Median: {:?}", median(&vec));
    println!("Mode: {:?}", mode(&vec).unwrap());
}

fn mean(sequence: &Vec<i32>) -> f64 { let mut sum: i32 = 0;
    for item in sequence {
       sum += item;
    }
    (sum as f64) / (sequence.len() as f64)
}

fn median(sequence: &Vec<i32>) -> i32 {
    let mut sequence_copy = sequence.clone();
    sequence_copy.sort();
    let index = sequence.len() / 2;
    sequence_copy[index]
}

fn mode(sequence: &Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    let mut max_count = 0;
    let mut result: Option<i32> = None;
    for item in sequence {
        let count_ref = counts.entry(item).or_insert(0);
        *count_ref += 1;
        if *count_ref > max_count {
            max_count = *count_ref;
            result = Some(*item);
        }
    };
    result
}
