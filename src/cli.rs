use crate::blockchain::Blockchain;
use std::env;

pub struct Cli;

impl Cli {
    pub fn run(chain: &mut Blockchain) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            match args[1].as_str() {
                "add" => {
                    if let Some(data) = args.get(2) {
                        chain.add(data.to_string());
                    }
                }
                "print" => {
                    let blocks = chain.get_blocks();
                    for block in blocks {
                        println!("{:?}", block);
                    }
                }
                "clear" => {
                    chain.clear();
                }
                _ => {
                    println!("Invalid command");
                }
            }
        }
    }
}
