mod errors;
mod hasm;
mod inst;
mod macros;
mod program;
mod vm;

pub use errors::*;
use hasm::{ha_to_hasm, hasm_to_ha};
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
    assembly,
    emulate,
    disassembly,
}

#[derive(EnumString, EnumIter, AsRefStr, Debug)]
#[allow(non_camel_case_types)]
pub enum ExecType {
    ha,
    hasm,
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
        Cmd::assembly => {
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
        Cmd::disassembly => {
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
            let available_exec_types: Vec<ExecType> = ExecType::iter().map(|ext| ext).collect();

            if args.len() < 3 {
                println!(
                    "Usage: input execution type, --* with the following {:#?}",
                    available_exec_types
                );
                exit(-1)
            }

            let exec_type = ExecType::from_str(&args[2].as_ref()).unwrap_or_else(|_| {
                println!(
                    "ERROR: invalid execution type, --* with the following {:#?}",
                    available_exec_types
                );
                exit(1);
            });

            if args.len() < 4 {
                println!("Usage: input file path");
                println!("\t*.ha");
                println!("\t*.hasm");
                exit(-1)
            }

            let exc_path = &args[3];
            let mut vm = VM::new();
            println!("Execution path: {}", exc_path);

            match exec_type {
                ExecType::ha => {
                    assert!(exc_path.ends_with(".ha"));

                    vm.load_ha_from_file(exc_path)?;
                }
                ExecType::hasm => {
                    assert!(exc_path.ends_with(".hasm"));

                    vm.load_hasm_from_file(exc_path)?;
                }
            }

            vm.run().map_err(io::Error::from)?;
            vm.dump();
        }
    };

    Ok(())
}
