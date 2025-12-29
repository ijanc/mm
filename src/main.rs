use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mm")]
#[command(version = "0.1.0")]
#[command(about = "my manager", long_about = None)]
struct Cli {
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// People (Team & Culture)
    ///
    /// Hiring, coaching, mentoring, career growth, performance management,
    /// psychological safety, fostering an inclusive culture, handling
    /// difficult conversations.
    People {
        #[command(subcommand)]
        cmd: PeopleCommands,
    },

    /// Product (Vision & Strategy)
    ///
    /// Understanding business goals, defining team vision, connecting work to
    /// company strategy, aligning with stakeholders, creating roadmaps.
    Product {
        #[command(subcommand)]
        cmd: ProductCommands,
    },
    
    /// Process (Execution & Delivery)
    /// 
    /// Planning, project management, feedback loops, metrics, operational
    /// excellence, risk management, efficient workflows, building scalable
    /// systems.
    Process {
        #[command(subcommand)]
        cmd: ProcessCommands,
    },

    /// Technology (Technical Leadership)
    /// 
    /// Ensuring quality, managing tech debt, driving innovation, architectural
    /// decisions, maintaining system health, understanding the tech landscape
    Technology {
        #[command(subcommand)]
        cmd: TechnologyCommands,
    },
}

#[derive(Subcommand)]
enum PeopleCommands {
    Team,
    Culture
}

#[derive(Subcommand)]
enum ProductCommands {
    Vision,
    Strategy
}

#[derive(Subcommand)]
enum ProcessCommands { }

#[derive(Subcommand)]
enum TechnologyCommands { }

fn main() {
    let cli = Cli::parse();

    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    match &cli.command {
        Commands::People { cmd } => match cmd {
            PeopleCommands::Team => println!(""),
            PeopleCommands::Culture=> println!(""),
        }
        Commands::Product { cmd } => match cmd {
            ProductCommands::Vision => println!(""),
            ProductCommands::Strategy => println!(""),
        }
        Commands::Process { cmd: _ } => todo!(),
        Commands::Technology { cmd: _ } => todo!(),
    }
}
