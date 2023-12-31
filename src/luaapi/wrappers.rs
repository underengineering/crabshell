use gtk::{
    cairo::Context,
    gdk::{ModifierType, RGBA},
    gio::{ApplicationFlags, SubprocessFlags},
    glib::GString,
    EventControllerScrollFlags,
};
use mlua::prelude::*;
use paste::paste;
use pulse::context::subscribe::InterestMaskSet;
use sysinfo::{CpuRefreshKind, ProcessRefreshKind, RefreshKind};

use crate::macros::{pack_mask, unpack_mask_postfixed};

macro_rules! bitmask_from_lua_impl {
    ($typ:ty, $default:expr, [$($value:ident),+]) => {
    paste! {
        pub struct [<$typ Wrapper>](pub $typ);
        impl<'lua> FromLua<'lua> for [<$typ Wrapper>] {
            fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
                Ok(match value {
                    LuaValue::Nil => Self(<$typ>::$default),
                    LuaValue::Table(table) => {
                        let mut flags = <$typ>::$default;
                        pack_mask!(
                            table,
                            flags,
                            $typ,
                            [$($value),+]
                        );
                        Self(flags)
                    }
                    value => Err(LuaError::FromLuaConversionError {
                        from: value.type_name(),
                        to: stringify!($typ),
                        message: None,
                    })?,
                })
            }
        }
    }
    };
}

pub struct GStringWrapper(pub GString);
impl<'lua> IntoLua<'lua> for GStringWrapper {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue> {
        Ok(LuaValue::String(lua.create_string(self.0.as_str())?))
    }
}

pub struct ModifierTypeWrapper(pub ModifierType);
impl<'lua> IntoLua<'lua> for ModifierTypeWrapper {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table_with_capacity(0, self.0.bits().count_ones() as usize)?;
        unpack_mask_postfixed!(
            table,
            self.0,
            ModifierType,
            [
                SHIFT, LOCK, CONTROL, ALT, BUTTON1, BUTTON2, BUTTON3, BUTTON4, BUTTON5, SUPER,
                HYPER, META
            ],
            _MASK
        );
        Ok(LuaValue::Table(table))
    }
}

pub struct ContextWrapper(pub Context);
impl<'lua> IntoLua<'lua> for ContextWrapper {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::UserData(lua.create_any_userdata(self.0)?))
    }
}

pub struct RGBAWrapper(pub RGBA);
impl<'lua> FromLua<'lua> for RGBAWrapper {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        if let LuaValue::Table(table) = value {
            let red = table.get::<_, Option<f32>>("r")?.unwrap_or(0.0);
            let green = table.get::<_, Option<f32>>("g")?.unwrap_or(0.0);
            let blue = table.get::<_, Option<f32>>("b")?.unwrap_or(0.0);
            let alpha = table.get::<_, Option<f32>>("a")?.unwrap_or(1.0);

            Ok(RGBAWrapper(RGBA::new(red, green, blue, alpha)))
        } else {
            Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "GdkRGBA",
                message: None,
            })
        }
    }
}

impl<'lua> IntoLua<'lua> for RGBAWrapper {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table_with_capacity(0, 4)?;
        table.set("r", self.0.red())?;
        table.set("g", self.0.green())?;
        table.set("b", self.0.blue())?;
        table.set("a", self.0.alpha())?;

        Ok(LuaValue::Table(table))
    }
}

pub struct CpuRefreshKindWrapper(pub CpuRefreshKind);
impl<'lua> FromLua<'lua> for CpuRefreshKindWrapper {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::Nil => Self(CpuRefreshKind::new()),
            LuaValue::Table(table) => {
                let mut kind = CpuRefreshKind::new();
                if table.get::<_, Option<bool>>("frequency")?.unwrap_or(false) {
                    kind = kind.with_frequency()
                }

                if table.get::<_, Option<bool>>("cpu_usage")?.unwrap_or(false) {
                    kind = kind.with_cpu_usage()
                }

                Self(kind)
            }
            value => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "CpuRefreshKind",
                message: None,
            })?,
        })
    }
}

pub struct ProcessRefreshKindWrapper(pub ProcessRefreshKind);
impl<'lua> FromLua<'lua> for ProcessRefreshKindWrapper {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::Nil => Self(ProcessRefreshKind::new()),
            LuaValue::Table(table) => {
                let mut kind = ProcessRefreshKind::new();
                if table.get::<_, Option<bool>>("cpu")?.unwrap_or(false) {
                    kind = kind.with_cpu();
                }

                if table.get::<_, Option<bool>>("disk_usage")?.unwrap_or(false) {
                    kind = kind.with_disk_usage();
                }

                if table.get::<_, Option<bool>>("user")?.unwrap_or(false) {
                    kind = kind.with_user();
                }

                Self(kind)
            }
            value => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "ProcessRefreshKind",
                message: None,
            })?,
        })
    }
}

pub struct RefreshKindWrapper(pub RefreshKind);
impl<'lua> FromLua<'lua> for RefreshKindWrapper {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::Nil => Self(RefreshKind::new()),
            LuaValue::Table(table) => {
                let mut kind = RefreshKind::new();
                macro_rules! with {
                    ($kind:ident) => {
                        if table
                            .get::<_, Option<bool>>(stringify!($kind))?
                            .unwrap_or(false)
                        {
                            paste! {
                                kind = kind.[<with_ $kind>]();
                            }
                        }
                    };
                }

                with!(networks);
                with!(networks_list);
                with!(disks);
                with!(disks_list);
                with!(memory);
                with!(components);
                with!(components_list);
                with!(users_list);

                if let Some(refresh_kind) =
                    table.get::<_, Option<ProcessRefreshKindWrapper>>("processes")?
                {
                    kind = kind.with_processes(refresh_kind.0);
                }

                if let Some(refresh_kind) = table.get::<_, Option<CpuRefreshKindWrapper>>("cpu")? {
                    kind = kind.with_cpu(refresh_kind.0);
                }

                Self(kind)
            }
            value => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "RefreshKind",
                message: None,
            })?,
        })
    }
}

bitmask_from_lua_impl!(
    SubprocessFlags,
    NONE,
    [
        STDIN_PIPE,
        STDIN_INHERIT,
        STDOUT_PIPE,
        STDOUT_SILENCE,
        STDERR_PIPE,
        STDERR_SILENCE,
        STDERR_MERGE,
        INHERIT_FDS
    ]
);

bitmask_from_lua_impl!(
    EventControllerScrollFlags,
    NONE,
    [VERTICAL, HORIZONTAL, DISCRETE, KINETIC, BOTH_AXES]
);

bitmask_from_lua_impl!(
    ApplicationFlags,
    FLAGS_NONE,
    [
        IS_SERVICE,
        IS_LAUNCHER,
        HANDLES_OPEN,
        HANDLES_COMMAND_LINE,
        SEND_ENVIRONMENT,
        NON_UNIQUE,
        CAN_OVERRIDE_APP_ID,
        ALLOW_REPLACEMENT,
        REPLACE
    ]
);

bitmask_from_lua_impl!(
    InterestMaskSet,
    NULL,
    [
        SINK,
        SOURCE,
        SINK_INPUT,
        SOURCE_OUTPUT,
        MODULE,
        CLIENT,
        SAMPLE_CACHE,
        SERVER,
        CARD,
        ALL
    ]
);
