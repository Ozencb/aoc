use md5;

fn produce_hash(base_input: &str, target_prefix: String) -> i32 {
    let mut counter = 0;

    return loop {
        let hash = md5::compute(format!("{}{}", base_input, counter));

        if format!("{:?}", hash).starts_with(&target_prefix) {
            break counter;
        }

        counter = counter + 1;
    };
}

fn main() {
    let base_input = "iwrupvqb";

    println!("{:?}", produce_hash(base_input, "0".repeat(5)));
    println!("{:?}", produce_hash(base_input, "0".repeat(6)));
}
