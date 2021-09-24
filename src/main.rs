use std::io;
use std::error::Error;

use drones_attack::game;

fn main() -> Result<(),Box<dyn Error>>{
    let mut stdout = io::stdout();

    game::play(&mut stdout)?;

    Ok(())
}




