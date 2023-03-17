use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author(clap::crate_authors!("\n")), version, about, long_about = clap::crate_description!())]
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
fn main() -> Result<(), failure::Error> {
    let args = Args::parse();
    let path = std::path::Path::new("./");
    let mut kv = kvs::KvStore::open(&path)?;
    match args.action {
        //         Action::Get(GetArgs { key }) => kv.get(key)?.and_then(|value| println!("{}", value)),
        Action::Get(GetArgs { key }) => {
            //             kv.get(key)?.and_then(|value| {
            //                 println!("{}", value);
            //                 Some(())
            //             });

            match kv.get(key)? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            };
        }
        Action::Set(SetArgs { key, value }) => kv.set(key, value)?,
        Action::Rm(RemoveArgs { key }) => match kv.remove(key) {
            Err(_) => {
                println!("Key not found");
                std::process::exit(1);
            }
            _ => {}
        },
    };
    Ok(())
}
