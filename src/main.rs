use clap::Parser;
use mlua::prelude::*;
use std::{env, fs, path::PathBuf};
use utils::LuaExt;

mod error;
mod hyprland;
mod luaapi;
mod macros;
mod system_info;
mod utils;

use crate::error::{Error, LuaErrorWrapper};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the lua file to execute.
    /// A default config path will be prepended if it's relative
    /// Defaults to $HOME/.config/cbar/main.lua
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Values to pass to lua script
    args: Vec<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config_path = args
        .config
        .map(|path| {
            if path.is_relative() {
                let mut abs_path =
                    PathBuf::from(env::var("HOME").expect("Failed to get the HOME variable"));
                abs_path.push(".config/cbar");
                abs_path.push(path);

                abs_path
            } else {
                path
            }
        })
        .unwrap_or_else(|| {
            let mut path =
                PathBuf::from(env::var("HOME").expect("Failed to get the HOME variable"));
            path.push(".config/cbar/main.lua");
            path
        });

    if !config_path.try_exists()? {
        return Err(Error::ConfigFileDoesNotExist);
    }

    if !config_path.is_file() {
        return Err(Error::ConfigFileNotAFile);
    }

    let lua = unsafe { Lua::new_with_stock_allocator() };
    lua.load_from_std_lib(LuaStdLib::ALL)?;

    let globals = lua.globals();
    let gtk_table = luaapi::gtk::add_api(&lua)?;
    let gio_table = luaapi::gio::add_api(&lua)?;
    let utils_table = luaapi::utils::add_api(&lua)?;
    let hyprland_table = luaapi::hyprland::add_api(&lua)?;
    let sysinfo_table = luaapi::sysinfo::add_api(&lua)?;
    let pulseaudio_table = luaapi::pulseaudio::add_api(&lua)?;
    let utf8_table = luaapi::utf8::add_api(&lua)?;
    let worker_table = luaapi::worker::add_api(&lua)?;
    globals.set("gtk", gtk_table)?;
    globals.set("gio", gio_table)?;
    globals.set("utils", utils_table)?;
    globals.set("hyprland", hyprland_table)?;
    globals.set("sysinfo", sysinfo_table)?;
    globals.set("pulseaudio", pulseaudio_table)?;
    globals.set("utf8", utf8_table)?;
    globals.set("worker", worker_table)?;

    // Set current directory to the config path
    env::set_current_dir(
        config_path
            .parent()
            .expect("Failed to get config parent directory"),
    )?;

    let config = fs::read_to_string(&config_path)?;
    let file_name = config_path.file_name().unwrap().to_str().unwrap();

    let lua_args = args
        .args
        .into_iter()
        .map(|x| lua.create_string(x).expect("Failed to create string"))
        .map(LuaValue::String);
    let result = lua
        .load(config)
        .set_name(file_name)
        .call::<_, ()>(LuaMultiValue::from_iter(lua_args));
    if let Err(lua_err) = result {
        eprintln!("{}", LuaErrorWrapper(lua_err));
        std::process::exit(1)
    }

    Ok(())
}
