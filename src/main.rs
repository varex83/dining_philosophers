mod fork;
mod philosopher;
mod runtime;

#[tokio::main]
async fn main() {
    let mut runtime = runtime::Runtime::new(5);

    runtime.run().await;

    // wait input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    runtime.stop().await;
}
