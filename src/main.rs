mod cmd;
mod recipe;

use cmd::{Cmd, SubCommand, StructOpt};
use recipe::*;

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        SubCommand::Ingredient(opt) => {
            inventory(&args.input, args.debug, &args.opt_type, &opt);
        }
        SubCommand::Recipe(opt) => {
            recipe(&args.input, args.debug, &args.opt_type, &opt);
        }
    }
}