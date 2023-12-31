use mlua::prelude::*;
use std::{fmt::Display, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lua error")]
    Lua(#[from] LuaError),
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Config file does not exist")]
    ConfigFileDoesNotExist,
    #[error("Config file is not a file")]
    ConfigFileNotAFile,
}

fn format_lua_error(err: &LuaError) -> String {
    match err {
        LuaError::SyntaxError {
            message,
            incomplete_input: _,
        } => {
            format!("Syntax error:\n{}", message)
        }
        LuaError::RuntimeError(err) => {
            format!("Runtime error:\n{}", err)
        }
        LuaError::CallbackError { traceback, cause } => {
            format!(
                "Callback error:\n{}\nCaused by: {}",
                traceback,
                format_lua_error(cause)
            )
        }
        LuaError::BadArgument {
            to,
            pos,
            name,
            cause,
        } => {
            format!(
                "Bad argument error:\n\tWrong argument {} passed to {}\nCaused by: {}",
                name.as_deref()
                    .map(|x| format!("`{}`", x))
                    .unwrap_or(format!("at {}", pos)),
                to.as_deref().unwrap_or("(unknown)"),
                format_lua_error(cause)
            )
        }
        LuaError::ToLuaConversionError { from, to, message } => {
            format!(
                "To lua conversion error:\n\tFailed to convert `{}` to `{}`{}",
                from,
                to,
                message
                    .as_deref()
                    .map(|x| format!(": {}", x))
                    .unwrap_or(String::new())
            )
        }
        LuaError::FromLuaConversionError { from, to, message } => {
            format!(
                "From lua conversion error:\n\tFailed to convert `{}` to `{}`{}",
                from,
                to,
                message
                    .as_deref()
                    .map(|x| format!(": {}", x))
                    .unwrap_or(String::new())
            )
        }
        LuaError::ExternalError(err) => {
            if let Some(lua_err) = err.downcast_ref::<LuaError>() {
                format!("External error:\n{}", LuaErrorWrapper(lua_err.clone()))
            } else {
                format!("External error:\n{:?}", err)
            }
        }
        err => {
            format!("Error: {:?}", err)
        }
    }
}

pub struct LuaErrorWrapper(pub LuaError);
impl Display for LuaErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_lua_error(&self.0))
    }
}
