pub mod require;

use mlua::prelude::*;

pub fn inject(luau: &Lua) -> LuaResult<()> {
    let globals = luau.globals();
    globals.set("require", luau.create_function(require::require)?)?;

    Ok(())
}
