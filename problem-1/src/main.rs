use problem_1::NaturalNumbers;

fn main() {
    let natural_numbers = NaturalNumbers::new();
    let natural_numbers = natural_numbers.set_multiples(vec![3, 5]);

    let mut multiples = vec![];

    for multiple in natural_numbers {
        if multiple < 1000 {
            multiples.push(multiple);
            continue;
        }
        break;
    }

    let sum: u32 = multiples.iter().sum();

    println!("The sum of all multiples of 3 and 5 under 1000 is {}", sum);
}
