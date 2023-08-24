mod block;
mod blockchain;
mod cli;
mod proof_of_work;

use blockchain::Blockchain;
use cli::Cli;

fn main() {
    let mut chain = Blockchain::new();

    Cli::run(&mut chain);
}
