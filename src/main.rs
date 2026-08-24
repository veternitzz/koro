mod cli;

use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // Do command line arguments
    let arg_return = cli::process()?;

    if arg_return == "none" {
        return Ok(())
    }

    let runtime = libkoro::runtime::new();
    runtime.open_globals()?;
    runtime.load_string(arg_return)?;

    Ok(())
}