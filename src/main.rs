use postgres_talk::hello_world;

#[tokio::main]
async fn main() {
    println!("{}",hello_world());
}
