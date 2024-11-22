use clap::Parser;

use catena::{
    chain::Chain,
    key,
    tx::{Op, Tx},
};
use ed25519_dalek::SigningKey;

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

    let mut rng = rand::rngs::OsRng;

    // TODO User abstraction.

    let mut admin = SigningKey::generate(&mut rng);

    let mut alice = SigningKey::generate(&mut rng);
    let mut bob = SigningKey::generate(&mut rng);
    let charlie_key = SigningKey::generate(&mut rng);

    let alice_addr = key::pub_to_hex(&alice.verifying_key());
    let bob_addr = key::pub_to_hex(&bob.verifying_key());
    let charlie_addr = key::pub_to_hex(&charlie_key.verifying_key());

    let mut chain = Chain::new(
        admin.verifying_key(),
        &vec![
            Tx::new(
                &mut admin,
                Op::Msg("Chancellor on brink of second bailout for banks".into()),
            )?,
            Tx::new(&mut admin, Op::SetDifficulty(cli.difficulty))?,
            Tx::new(
                &mut admin,
                Op::Coinbase {
                    dst: alice_addr.clone(),
                    amount: 10,
                },
            )?,
            Tx::new(
                &mut admin,
                Op::Coinbase {
                    dst: bob_addr.clone(),
                    amount: 10,
                },
            )?,
            Tx::new(
                &mut admin,
                Op::Coinbase {
                    dst: charlie_addr.clone(),
                    amount: 10,
                },
            )?,
        ][..],
    )?;

    chain.submit(
        &vec![Tx::new(
            &mut alice,
            Op::Pay {
                dst: bob_addr.clone(),
                amount: 10,
            },
        )?][..],
    )?;
    chain.submit(
        &vec![Tx::new(
            &mut bob,
            Op::Pay {
                dst: charlie_addr.clone(),
                amount: 10,
            },
        )?][..],
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
