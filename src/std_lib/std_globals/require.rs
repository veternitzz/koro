use std::{format, fs};

use crate::std_lib::{self, *};

use mlua::prelude::*;

pub fn require(luau: &Lua, path: String) -> LuaResult<LuaValue> {
    if path.starts_with("@std") {
        return Ok(get_std_library(luau, path).unwrap().into_lua(&luau)?)

    } else {
        let library = get_path_library(path)?;
        return Ok(library)
    }
}

fn get_std_library(luau: &Lua, path: String) -> LuaResult<LuaTable> {
    match path.as_str() {
        "@std" => Ok(std_lib::create(&luau)?),
        "@std/fs" => Ok(std_fs::create(&luau)?),
        "@std/io" => Ok(std_io::create(&luau)?),
        "@std/sys" => Ok(std_sys::create(&luau)?),
        "@std/fmt" => Ok(std_fmt::create(&luau)?),
        &_ => Err(LuaError::RuntimeError(String::from(format!("invalid standard library: {}", path))))
    }
}

fn get_path_library(path: String) -> LuaResult<LuaValue> {
    let library_state = Lua::new();
    let library_chunk = fs::read_to_string(path)?;

    Ok(library_state.load(library_chunk).call(())?)
}