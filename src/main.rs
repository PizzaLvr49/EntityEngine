use mlua::{Lua, Result};
use std::time::Instant;

fn main() -> Result<()> {
    let lua = Lua::new();

    let length_fast = lua.create_function(|_, (x, y): (f64, f64)| Ok((x * x + y * y).sqrt()))?;

    lua.globals().set("length_fast", length_fast)?;

    // Warm up
    for _ in 0..10000 {
        let _: f64 = lua.load("return length_fast(1, 1)").eval()?;
    }

    let script = r#"
        local sum = 0.0
        for i = 1, 10_000_000 do
            sum += length_fast(10, 20)
        end
        return sum
    "#;

    let start = Instant::now();
    let result: f64 = lua.load(script).eval()?;
    let duration = start.elapsed();

    println!("Result from Luau + Rust: {}", result);
    println!("Total time: {:?}", duration);
    println!("Time per iteration: {:?}", duration / 10_000_000);
    println!(
        "Iterations per second: {:.0}",
        10_000_000.0 / duration.as_secs_f64()
    );

    Ok(())
}
