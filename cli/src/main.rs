const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("lmk {}", VERSION.unwrap_or("unknown ver."));
        println!("usage: {} <SUMMARY> [BODY]", args[0]);

        return;
    }

    let backend_url = option_env!("LMK_BACKEND_URL").unwrap_or("https://localhost");

    println!(
        "Hello, world! Sending '{}' with summary '{}'",
        args[1],
        args.get(2).unwrap_or(&"No summary :(".into())
    );
}
