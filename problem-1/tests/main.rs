use cucumber::World;

#[derive(Default, Debug, World)]
struct ProblemWorld {}

#[tokio::main]
async fn main() {
    ProblemWorld::cucumber()
        .fail_on_skipped()
        .run("./tests/features/problem-1.feature")
        .await;
}
