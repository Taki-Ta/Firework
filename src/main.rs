mod types;
mod particle;
mod firework;
mod app;

use std::io::Result;

fn main() -> Result<()> {
    app::run()
}
