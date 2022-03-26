use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "RutuduFD")]
#[clap(author = "Foom <lordfoom@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Searches through collections of rutudu lists", long_about = None)]
struct Args{
    #[clap(required = true, parse(from_os_str))]
    term: Vec<String>,
}

fn main() {
    println!("Hello, world!");
}
