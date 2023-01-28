use clap::Parser;
use rpmvercmp::rpmvercmp;

#[derive(Parser, Debug)]
#[command(version = option_env!("VERGEN_GIT_SEMVER").unwrap_or(env!("CARGO_PKG_VERSION")))]
/// Compare A and B which version string is newer.
struct Cli {
    a: String,
    b: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let (a, b) = (args.a, args.b);

    use std::cmp::Ordering;
    let r = rpmvercmp(&a, &b)?;
    let s = match r {
        Ordering::Equal => format!("{a} and {b} are the same version"),
        Ordering::Greater => format!("{a} is newer than {b}"),
        Ordering::Less => format!("{b} is newer than {a}"),
    };
    println!("{}", &s);
    Ok(())
}
