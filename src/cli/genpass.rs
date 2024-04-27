use crate::{process_genpass, CmdExector};
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    // cargo run -- genpass -l 32
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub numbers: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbol,
        )?;
        println!("{}", password);
        let estimate = zxcvbn(&password, &[]).unwrap();
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
