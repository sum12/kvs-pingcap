use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Action
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// GET operation
    Get(GetArgs),
    /// SET operation
    Set(SetArgs),
    /// Remove Operation
    Rm(RemoveArgs),
}

#[derive(clap::Args, Debug)]
struct GetArgs {
    /// Key to use for get operation
    #[arg()]
    key: String,
}

#[derive(clap::Args, Debug)]
struct SetArgs {
    /// Key to use for set operation
    #[arg()]
    key: String,
    /// Value to use for KEY
    #[arg()]
    value: String,
}

#[derive(clap::Args, Debug)]
struct RemoveArgs {
    /// Key to use for remove operation
    #[arg()]
    key: String,
}
fn main() -> Result<(), u8> {
    let args = Args::parse();
    match args.action {
        Action::Get(GetArgs { .. }) => panic!("unimplemented"),
        Action::Set(SetArgs { .. }) => panic!("unimplemented"),
        Action::Rm(RemoveArgs { .. }) => panic!("unimplemented"),
    }

    Ok(())
}
