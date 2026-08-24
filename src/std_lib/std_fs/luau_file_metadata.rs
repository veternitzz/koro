use std::fs;

use mlua::prelude::*;

pub struct LuauFileMetadata {
    pub metadata: fs::Metadata
}

impl LuauFileMetadata {
    pub fn size(&self) -> LuaResult<f64> {
        Ok(self.metadata.len() as f64)
    }
}

impl LuaUserData for LuauFileMetadata {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("size", |_, this, ()| -> LuaResult<f64> {
            Ok(this.size()?)
        });
    }
}