use std::fs;

fn main() {
    let filename = "data.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong");

    let lines = contents.lines();
    let line_count = lines.clone().count();

    let mut gift_list = vec![vec![0; 3]; line_count];

    for line in lines {
        let mut sizes = line
            .split('x')
            .map(|size| size.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        sizes.sort();

        gift_list.push(sizes);
    }

    let total_area: i32 = gift_list
        .clone()
        .into_iter()
        .map(|v| calculate_area(v))
        .sum();
    let ribbon_length: i32 = gift_list
        .clone()
        .into_iter()
        .map(|v| get_ribbon_size(&v))
        .sum();

    println!(
        "Wrapping paper needed: {}\nRibbon needed: {}",
        total_area, ribbon_length
    );
}

// (2*l*w + 2*w*h + 2*h*l) + smallest side's area
fn calculate_area(v: Vec<i32>) -> i32 {
    let smallest_side = v[0] * v[1]; // since vector is now sorted, we can use first two elements for smallest side
    (2 * v[0] * v[1]) + (2 * v[0] * v[2]) + (2 * v[1] * v[2]) + smallest_side
}

fn get_ribbon_size(v: &Vec<i32>) -> i32 {
    get_smallest_perimeter(v) + get_volume(v)
}

fn get_smallest_perimeter(v: &Vec<i32>) -> i32 {
    (v[0] * 2) + (v[1] * 2)
}

fn get_volume(v: &Vec<i32>) -> i32 {
    v[0] * v[1] * v[2]
}
