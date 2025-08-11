use mlua::{Error, FromLua, Function, Lua, Result};
use std::time::Instant;

struct BenchmarkResult {
    result: f64,
    operations: u32,
    max_value: f64,
}

impl FromLua for BenchmarkResult {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> Result<Self> {
        match value {
            mlua::Value::Table(table) => {
                let result: f64 =
                    table
                        .get("result")
                        .map_err(|e| Error::FromLuaConversionError {
                            from: "Table",
                            to: "BenchmarkResult".to_string(),
                            message: Some(format!("Failed to get 'result' field: {}", e)),
                        })?;

                let operations: u32 =
                    table
                        .get("operations")
                        .map_err(|e| Error::FromLuaConversionError {
                            from: "Table",
                            to: "BenchmarkResult".to_string(),
                            message: Some(format!("Failed to get 'operations' field: {}", e)),
                        })?;

                let max_value: f64 =
                    table
                        .get("max_value")
                        .map_err(|e| Error::FromLuaConversionError {
                            from: "Table",
                            to: "BenchmarkResult".to_string(),
                            message: Some(format!("Failed to get 'max_value' field: {}", e)),
                        })?;

                Ok(BenchmarkResult {
                    result,
                    operations,
                    max_value,
                })
            }
            _ => Err(Error::FromLuaConversionError {
                from: value.type_name(),
                to: "BenchmarkResult".to_string(),
                message: Some(
                    "Expected a table with result, operations, and max_value fields".to_string(),
                ),
            }),
        }
    }
}

fn main() -> Result<()> {
    let lua = Lua::new();

    let length_fast: Function =
        lua.create_function(|_, (x, y): (f64, f64)| Ok((x * x + y * y).sqrt()))?;

    let distance: Function = lua.create_function(|_, (x1, y1, x2, y2): (f64, f64, f64, f64)| {
        let dx = x2 - x1;
        let dy = y2 - y1;
        Ok((dx * dx + dy * dy).sqrt())
    })?;

    let complex_calc: Function = lua.create_function(|_, (angle, radius): (f64, f64)| {
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let magnitude = (x * x + y * y).sqrt();

        if magnitude > 10.0 {
            Ok(magnitude * 1.5 + angle.tan().abs())
        } else {
            Ok(magnitude * 0.8 + angle.sin())
        }
    })?;
    lua.globals().set("length_fast", length_fast)?;
    lua.globals().set("distance", distance)?;
    lua.globals().set("complex_calc", complex_calc)?;

    for i in 0..5000 {
        let angle = i as f64 * 0.001;
        let _: f64 = lua.load("return length_fast(1, 1)").eval()?;
        let _: f64 = lua.load("return distance(0, 0, 3, 4)").eval()?;
        let _: f64 = lua
            .load(&format!("return complex_calc({}, 15)", angle))
            .eval()?;
    }

    let script: &'static str = r#"
        local sum = 0.0
        local operations = 0
        local max_value = 0.0
        local pi = 3.14159265359
        
        for i = 1, 5000000 do
            local angle = (i % 628) * 0.01
            local radius = 10 + (i % 20)
            
            local len = length_fast(radius * 0.5, radius * 0.3)
            local dist = distance(0, 0, len, angle)
            local complex = complex_calc(angle, radius)
            
            local combined = len + dist + complex
            sum = sum + combined
            operations = operations + 3
            
            if combined > max_value then
                max_value = combined
            end
            
            if i % 1000 == 0 then
                sum = sum * 1.0001
            end
        end
        
        return { 
            result = sum,
            operations = operations,
            max_value = max_value
        }
    "#;

    println!("Starting enhanced benchmark...");
    let start = Instant::now();
    let result: BenchmarkResult = lua.load(script).eval()?;
    let duration = start.elapsed();

    println!("Result: {}", result.result);
    println!("Total operations: {}", result.operations);
    println!("Maximum value encountered: {}", result.max_value);
    println!("Total time: {:?}", duration);
    println!("Time per iteration: {:?}", duration / 5_000_000);
    println!("Time per operation: {:?}", duration / result.operations);
    println!(
        "Iterations per second: {:.0}",
        5_000_000.0 / duration.as_secs_f64()
    );
    println!(
        "Operations per second: {:.0}",
        result.operations as f64 / duration.as_secs_f64()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_functions() -> Result<()> {
        let lua = Lua::new();

        let length_fast: Function =
            lua.create_function(|_, (x, y): (f64, f64)| Ok((x * x + y * y).sqrt()))?;

        lua.globals().set("length_fast", length_fast)?;

        let result: f64 = lua.load("return length_fast(3, 4)").eval()?;
        assert!((result - 5.0).abs() < 1e-10);

        Ok(())
    }

    #[test]
    fn test_benchmark_result_parsing() -> Result<()> {
        let lua = Lua::new();

        let script = r#"
            return {
                result = 42.5,
                operations = 1000,
                max_value = 99.9
            }
        "#;

        let result: BenchmarkResult = lua.load(script).eval()?;
        assert_eq!(result.result, 42.5);
        assert_eq!(result.operations, 1000);
        assert_eq!(result.max_value, 99.9);

        Ok(())
    }
}
