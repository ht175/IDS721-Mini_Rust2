use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Rachel",
    about = "Chicken and Rabbit in same case problem"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rachel")]
    ChickenRabbitSameCage {
        #[clap(short, long)]
        thead: i32,
        leg: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::ChickenRabbitSameCage { thead, leg }) => {
            let chicken = myproject::get_chicken(thead, leg);
            if chicken == -1 {
                println!("No solution");
            } else {
                let rabbit: i32 = myproject::get_rabbit(chicken, thead);
                println!("chicken{}, rabbit{}", chicken, rabbit);
            }
        }
        None => println!("No subcommand was used"),
    }
}
