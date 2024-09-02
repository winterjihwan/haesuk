mod dehasm;
mod errors;
mod hasm;
mod inst;
mod macros;
mod program;
mod vm;

use dehasm::ha_to_hasm;
pub use errors::*;
use hasm::hasm_to_ha;
use std::{
    env,
    io::{self},
    process::exit,
    str::FromStr,
};
use strum::IntoEnumIterator;
pub use strum_macros::EnumString;
use strum_macros::{AsRefStr, EnumIter};
pub use vm::*;

#[derive(EnumString, EnumIter, AsRefStr, Debug)]
#[allow(non_camel_case_types)]
pub enum Cmd {
    hasm,
    emulate,
    dehasm,
}

fn main() -> io::Result<()> {
    let avaiable_cmds: Vec<Cmd> = Cmd::iter().map(|cmd| cmd).collect();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: input cmd, --* with the following {:#?}",
            avaiable_cmds
        );
        exit(-1)
    }

    let cmd = Cmd::from_str(&args[1].as_ref()).unwrap_or_else(|_| {
        println!(
            "ERROR: invalid cmd, --* with the following {:#?}",
            avaiable_cmds
        );
        exit(1);
    });

    match cmd {
        Cmd::hasm => {
            if args.len() < 3 {
                println!("Usage: input path args");
                println!("\t*.hasm");
                exit(-1)
            }

            let hasm_path = &args[2];
            assert!(hasm_path.ends_with(".hasm"));
            println!("Hasm path: {}", hasm_path);

            hasm_to_ha(hasm_path)?;
        }

        Cmd::dehasm => {
            if args.len() < 3 {
                println!("Usage: input path args");
                println!("\t*.ha");
                exit(-1)
            }

            let hasm_path = &args[2];
            assert!(hasm_path.ends_with(".ha"));
            println!("Ha path: {}", hasm_path);

            ha_to_hasm(hasm_path)?;
        }

        Cmd::emulate => {
            if args.len() < 3 {
                println!("Usage: input path args");
                println!("\t*.ha");
                println!("Extra optional args: limit");
                println!("\tlimit __");
                exit(-1)
            }

            let maybe_limit = if args.len() > 3 {
                if args.len() < 5 {
                    println!("Usage: input limit");
                    println!("\t0~2^16");
                    exit(-1)
                }

                Some(args[4].parse::<u16>().unwrap())
            } else {
                None
            };

            let eml_path = &args[2];
            assert!(eml_path.ends_with(".ha"));

            let mut vm = VM::new();
            vm.load_ha_from_file(eml_path)?;
            vm.run(maybe_limit).map_err(io::Error::from)?;
            vm.dump();
        }
    };

    Ok(())
}
