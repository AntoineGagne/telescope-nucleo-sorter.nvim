use mimalloc::MiMalloc;
use mlua::{
    prelude::{Lua, LuaResult, LuaString, LuaTable, LuaUserDataMethods},
    FromLua, UserData, Value,
};
use nucleo_matcher::pattern::{CaseMatching, Normalization};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

struct Matcher {
    matcher: nucleo_matcher::Matcher,
    pattern: Option<nucleo_matcher::pattern::Pattern>,
    options: MatchOptions,
}

impl Default for Matcher {
    fn default() -> Self {
        Matcher {
            matcher: nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths()),
            pattern: None,
            options: MatchOptions::default(),
        }
    }
}

impl UserData for Matcher {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "set_pattern",
            |_, this: &mut Matcher, pattern: LuaString| {
                if let Some(old) = this.pattern.as_mut() {
                    old.reparse(
                        pattern.to_str()?,
                        this.options.case_mode,
                        this.options.normalization,
                    );
                } else {
                    this.pattern = Some(nucleo_matcher::pattern::Pattern::parse(
                        pattern.to_str()?,
                        this.options.case_mode,
                        this.options.normalization,
                    ));
                };
                Ok(())
            },
        );
        methods.add_method_mut("match", |_, this: &mut Self, (str,): (LuaString,)| {
            this.pattern
                .as_ref()
                .map(|pattern| {
                    if pattern.atoms.is_empty() {
                        return (1, vec![]);
                    }

                    let as_bytes = str.as_bytes();
                    let ascii_encoded = nucleo_matcher::Utf32Str::Ascii(as_bytes);
                    let mut results = vec![];
                    pattern
                        .indices(ascii_encoded, &mut this.matcher, &mut results)
                        .map(|score| (score, results.iter().map(|x| x + 1).collect()))
                        .unwrap_or_else(|| (0, results))
                })
                .ok_or_else(|| mlua::Error::runtime("[nucleo_matcher]: Pattern is empty"))
        });
    }
}

struct MatchOptions {
    case_mode: CaseMatching,
    normalization: Normalization,
}

impl Default for MatchOptions {
    fn default() -> Self {
        MatchOptions {
            case_mode: CaseMatching::Smart,
            normalization: Normalization::Smart,
        }
    }
}

impl FromLua<'_> for MatchOptions {
    fn from_lua<'lua>(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match value {
            Value::Table(table) => {
                let case_mode = table.get("case_mode").unwrap_or("smart".to_owned());
                let normalization = table.get("normalize_mode").unwrap_or("smart".to_owned());
                Ok(MatchOptions {
                    case_mode: match case_mode.as_str() {
                        "smart" => CaseMatching::Smart,
                        "ignore" => CaseMatching::Ignore,
                        "respect" => CaseMatching::Respect,
                        other => Err(mlua::Error::runtime(format!(
                            "[nucleo]: {other} is not a valid `case_mode` option"
                        )))?,
                    },
                    normalization: match normalization.as_str() {
                        "smart" => Normalization::Smart,
                        "never" => Normalization::Never,
                        other => Err(mlua::Error::runtime(format!(
                            "[nucleo]: {other} is not a valid `normalization` option"
                        )))?,
                    },
                })
            }
            _ => Err(mlua::Error::runtime("expected table for options")),
        }
    }
}

#[mlua::lua_module(skip_memory_check)]
fn nucleo_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "create_matcher",
        lua.create_function(|_, options: MatchOptions| {
            let matcher = Matcher {
                options,
                ..Matcher::default()
            };
            Ok((matcher,))
        })?,
    )?;
    Ok(exports)
}
