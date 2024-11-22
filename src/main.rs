use clap::Parser;

use catena::{chain::Chain, tx::Tx};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, default_value_t = 4)]
    difficulty: usize,

    #[clap(short = 'c', long)]
    log_color: bool,

    #[clap(short, long, default_value_t = tracing::Level::DEBUG)]
    log_level: tracing::Level,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    catena::tracing_init(cli.log_level, cli.log_color)?;
    tracing::debug!(?cli, "Starting.");

    let mut chain = Chain::new(
        &vec![
            Tx::Msg("Chancellor on brink of second bailout for banks".into()),
            Tx::SetDifficulty(cli.difficulty),
            Tx::Coinbase {
                dst: "Alice".into(),
                amount: 10,
            },
            Tx::Coinbase {
                dst: "Bob".into(),
                amount: 10,
            },
            Tx::Coinbase {
                dst: "Charlie".into(),
                amount: 10,
            },
        ][..],
    )?;

    chain.submit(
        &vec![Tx::Pay {
            src: "Alice".into(),
            dst: "Bob".into(),
            amount: 10,
        }][..],
    )?;
    chain.submit(
        &vec![Tx::Pay {
            src: "Bob".into(),
            dst: "Charlie".into(),
            amount: 5,
        }][..],
    )?;

    tracing::debug!("Validating chain.");
    let is_valid = chain.is_valid()?;
    assert!(is_valid);
    tracing::debug!("Chain is valid.");

    tracing::debug!("Showing chain.");
    for block in &chain.blocks {
        dbg!(block.height, &block.hash);
    }
    dbg!(chain.ledger);
    Ok(())
}
