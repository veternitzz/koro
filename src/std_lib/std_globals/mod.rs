pub mod require;

use mlua::prelude::*;

pub fn inject(luau: &Lua) -> LuaResult<()> {
    let globals = luau.globals();
    globals.set("require", luau.create_function(require::require)?)?;

    Ok(())
}

pub fn create(luau: &Lua, chunk: LuaChunk) -> LuaResult<()> {
    let globals_env = luau.create_table()?;
    globals_env.set("require", luau.create_function(require::require)?)?;
    globals_env.set_readonly(true);

    chunk.set_environment(globals_env);
    Ok(())
}