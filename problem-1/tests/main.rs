use cucumber::{gherkin::Step, given, then, World};
use problem_1::NaturalNumbers;

#[derive(Default, Debug, World)]
struct ProblemWorld {
    top: u32,
    multiples: Vec<u32>,
    natural_numbers: NaturalNumbers,
}

#[given(expr = "all natural numbers below {int}")]
fn given_natural_number_bellow(world: &mut ProblemWorld, top: u32) {
    world.natural_numbers = NaturalNumbers::new();
    world.top = top;
}

#[given(expr = "a list of numbers")]
fn given_list_of_numbers(world: &mut ProblemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let multiples: Vec<u32> = table
            .rows
            .iter()
            .map(|row| row[0].parse().unwrap())
            .collect();

        world.natural_numbers = world.natural_numbers.set_multiples(multiples);
    }
}

#[then(expr = "the multiples of those numbers under the natural are")]
fn then_multiples_from_numbers(world: &mut ProblemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let rows = table
            .rows
            .iter()
            .map(|row| row[0].parse().unwrap())
            .collect::<Vec<u32>>();

        for multiple in world.natural_numbers.by_ref() {
            if multiple < world.top {
                world.multiples.push(multiple);
                continue;
            }
            break;
        }
        assert_eq!(rows, world.multiples);
    }
}

#[then(expr = "the sum of these numbers is {int}")]
fn then_sum_of_multiples(world: &mut ProblemWorld, sum: u32) {
    assert_eq!(world.multiples.iter().sum::<u32>(), sum);
}

#[tokio::main]
async fn main() {
    ProblemWorld::cucumber()
        .fail_on_skipped()
        .run("./tests/features/problem-1.feature")
        .await;
}
