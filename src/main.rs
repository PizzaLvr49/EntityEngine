use mlua::{Error, FromLua, Function, Lua, Result};
use std::time::Instant;
struct ReturnValue {
    result: f64,
}

impl FromLua for ReturnValue {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> Result<Self> {
        match value {
            mlua::Value::Table(table) => {
                let result: f64 =
                    table
                        .get("result")
                        .map_err(|e| Error::FromLuaConversionError {
                            from: "Table",
                            to: "ReturnValue".to_string(),
                            message: Some(format!("Failed to get 'result' field: {}", e)),
                        })?;
                Ok(ReturnValue { result })
            }
            _ => Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ReturnValue".to_string(),
                message: Some("Expected a table with 'result' field".to_string()),
            }),
        }
    }
}

fn main() -> Result<()> {
    let lua = Lua::new();

    let length_fast: Function =
        lua.create_function(|_, (x, y): (f64, f64)| Ok((x * x + y * y).sqrt()))?;

    lua.globals().set("length_fast", length_fast)?;

    // Warm up
    for _ in 0..10000 {
        let _: f64 = lua.load("return length_fast(1, 1)").eval()?;
    }

    let script: &'static str = r#"
        local sum = 0.0
        for i = 1, 10000000 do
            sum = sum + length_fast(10, 20)
        end
        return { result = sum }
    "#;

    let start = Instant::now();
    let result: ReturnValue = lua.load(script).eval()?;
    let duration = start.elapsed();

    println!("Result from Luau + Rust: {}", result.result);
    println!("Total time: {:?}", duration);
    println!("Time per iteration: {:?}", duration / 10_000_000);
    println!(
        "Iterations per second: {:.0}",
        10_000_000.0 / duration.as_secs_f64()
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_entity_engine() {}
}
