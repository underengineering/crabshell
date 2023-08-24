use gtk::glib;
use mlua::prelude::*;
use std::time::Duration;

fn add_grass_api(lua: &Lua, utils_table: &LuaTable) -> LuaResult<()> {
    utils_table.set(
        "scss_from_path",
        lua.create_function(|_, path: String| {
            grass::from_path(path, &grass::Options::default()).into_lua_err()
        })?,
    )?;

    utils_table.set(
        "scss_from_string",
        lua.create_function(|_, input: String| {
            grass::from_string(input, &grass::Options::default()).into_lua_err()
        })?,
    )?;

    Ok(())
}

fn add_icons_api(lua: &Lua, utils_table: &LuaTable) -> LuaResult<()> {
    utils_table.set(
        "lookup_icon",
        lua.create_function(|_, (name, options): (String, Option<LuaTable>)| {
            let mut builder = freedesktop_icons::lookup(&name);
            let mut theme = None;
            if let Some(options) = options {
                if let Some(size) = options.get::<_, Option<u16>>("size")? {
                    builder = builder.with_size(size);
                }

                if let Some(scale) = options.get::<_, Option<u16>>("scale")? {
                    builder = builder.with_scale(scale);
                }

                if let Some(theme_inner) = options.get::<_, Option<String>>("theme")? {
                    // Move it to prevent borrow issues
                    theme = Some(theme_inner);
                }

                if options.get::<_, Option<bool>>("cache")?.unwrap_or(false) {
                    builder = builder.with_cache();
                }

                if options
                    .get::<_, Option<bool>>("force_svg")?
                    .unwrap_or(false)
                {
                    builder = builder.force_svg();
                }
            }

            if let Some(ref theme) = theme {
                builder = builder.with_theme(theme);
            }

            if let Some(path) = builder.find() {
                let path_str = path.into_os_string().into_string().unwrap();
                Ok(Some(path_str))
            } else {
                Ok(None)
            }
        })?,
    )?;

    Ok(())
}

fn add_tokio_api(lua: &Lua, utils_table: &LuaTable) -> LuaResult<()> {
    utils_table.set(
        "sleep",
        lua.create_async_function(|_, secs: f64| async move {
            glib::timeout_future(Duration::from_secs_f64(secs)).await;
            Ok(())
        })?,
    )?;

    Ok(())
}

fn add_other_api(lua: &Lua, utils_table: &LuaTable) -> LuaResult<()> {
    utils_table.set(
        "print_table",
        lua.load(
            r#"
            local tbl, seen, depth = ...
            seen = seen or {}
            depth = depth or 0

            seen[tbl] = true

            io.write(("\t"):rep(depth))
            io.write("{\n")
            for k, v in pairs(tbl) do
                local k_formatted
                local ktype = type(k)
                if ktype == "string" then
                    k_formatted = ("\"%s\""):format(k:gsub("\n", "\\n"))
                else
                    k_formatted = k
                end

                local v_formatted
                local vtype = type(v)
                if vtype == "table" and not seen[v] then
                    io.write(("\t"):rep(depth + 1))
                    io.write(("[%s] =\n"):format(k_formatted))
                    utils.print_table(v, seen, depth + 1)
                    io.write(",\n")
                    goto continue
                elseif vtype == "string" then
                    v_formatted = ("\"%s\""):format(k:gsub("\n", "\\n"))
                else
                    v_formatted = v
                end

                io.write(("\t"):rep(depth + 1))
                io.write(("[%s] = %s,\n"):format(k_formatted, v_formatted))
                ::continue::
            end
            io.write(("\t"):rep(depth))
            io.write("}")
            "#,
        )
        .into_function()?,
    )?;

    Ok(())
}

pub fn add_api(lua: &Lua) -> LuaResult<LuaTable> {
    let utils_table = lua.create_table()?;

    add_grass_api(lua, &utils_table)?;
    add_icons_api(lua, &utils_table)?;
    add_tokio_api(lua, &utils_table)?;
    add_other_api(lua, &utils_table)?;

    Ok(utils_table)
}
