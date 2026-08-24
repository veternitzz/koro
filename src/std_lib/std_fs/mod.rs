pub mod luau_file_metadata;

use std::fs;

use mlua::prelude::*;

use luau_file_metadata::LuauFileMetadata;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let table = luau.create_table()?;
    table.set("read", luau.create_function(fs_read)?)?;
    table.set("writeFile", luau.create_function(fs_write_file)?)?;
    table.set("writeDir", luau.create_function(fs_write_dir)?)?;
    table.set("exists", luau.create_function(fs_exists)?)?;
    table.set("writeDirAll", luau.create_function(fs_write_dir_all)?)?;
    table.set("removeFile", luau.create_function(fs_remove_file)?)?;
    table.set("removeDir", luau.create_function(fs_remove_dir)?)?;
    table.set("removeDirAll", luau.create_function(fs_remove_dir_all)?)?;
    table.set("metadata", luau.create_function(fs_metadata)?)?;

    table.set_readonly(true);

    Ok(table)
}

fn fs_read(luau: &Lua, (kind, path): (String, String)) -> LuaResult<LuaEither<LuaString, LuaTable>> {
    if fs::exists(&path)? != true {
        return Err(mlua::Error::runtime("invalid path at fs.read"))
    }

    if kind == "f" {
        let contents = fs::read_to_string(&path)?;
        let contents_as_luau_string = luau.create_string(contents.as_bytes())?;

        return Ok(LuaEither::Left(contents_as_luau_string))
    } else if kind == "d" {
        let entries = fs::read_dir(&path)?;
        let contents = luau.create_table()?;
        
        for file in entries {
            let entry = file?.file_name().into_string().unwrap();
            let entry_as_luau_string = luau.create_string(entry.as_bytes())?;

            contents.set(1, entry_as_luau_string)?;
        }

        return Ok(LuaEither::Right(contents))
    }

    return Err(mlua::Error::runtime("invalid file read kind at fs.read"))
} 

fn fs_write_file(_: &Lua, (path, contents): (String, String)) -> LuaResult<()> {
    fs::write(path, contents)?;
    Ok(())
}

fn fs_write_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::create_dir(path)?;
    Ok(())
}

fn fs_exists(_: &Lua, path: String) -> LuaResult<bool> {
    fs::exists(path).into_lua_err()
}

fn fs_write_dir_all(_: &Lua, path: String) -> LuaResult<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

fn fs_remove_file(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_file(path)?;
    Ok(())
}

fn fs_remove_dir(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_dir(path)?;
    Ok(())
}

fn fs_remove_dir_all(_: &Lua, path: String) -> LuaResult<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

fn fs_metadata(_: &Lua, path: String) -> LuaResult<LuauFileMetadata> {
    if fs::exists(&path)? != true {
        return Err(mlua::Error::runtime("invalid path at fs.metadata"))
    }

    Ok(LuauFileMetadata { metadata: fs::metadata(path)? })
}