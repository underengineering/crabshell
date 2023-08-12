use std::{env, path::PathBuf};

use clap::Parser;
use gtk::Application;

use mlua::prelude::*;
use std::fs;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

mod error;
mod hyprland;
mod luaapi;
mod system_info;

use crate::error::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the lua file to execute. Defaults to $HOME/.config/cbar/main.lua
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config_path = args.config.unwrap_or_else(|| {
        let mut path = PathBuf::from(env::var("HOME").expect("Failed to get the HOME variable"));
        path.push(".config/cbar/main.lua");
        path
    });

    if !config_path.try_exists()? {
        return Err(Error::ConfigFileDoesNotExist);
    }

    if !config_path.is_file() {
        return Err(Error::ConfigFileNotAFile);
    }

    let lua = unsafe { Lua::unsafe_new() };
    lua.load_from_std_lib(LuaStdLib::ALL)?;

    let globals = lua.globals();
    let gtk_table = luaapi::gtk::add_api(&lua)?;
    let utils_table = luaapi::utils::add_api(&lua)?;
    let hyprland_table = luaapi::hyprland::add_api(&lua)?;
    let sysinfo_table = luaapi::sysinfo::add_api(&lua)?;
    globals.set("utils", utils_table)?;
    globals.set("hyprland", hyprland_table)?;
    globals.set("sysinfo", sysinfo_table)?;

    // Set current directory to the config path
    env::set_current_dir(
        config_path
            .parent()
            .expect("Failed to get config parent directory"),
    )?;

    let app = Application::builder().application_id(APP_ID).build();

    gtk_table.set("app", lua.create_any_userdata(app)?)?;
    globals.set("gtk", gtk_table)?;

    let config = fs::read_to_string(&config_path)?;
    let file_name = config_path.file_name().unwrap().to_str().unwrap();
    lua.load(config).set_name(file_name).exec()?;

    Ok(())
}
