use std::error::Error;
use std::io;
use std::io::Stdout;
use std::sync::mpsc::{self};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};

use crate::frame;
use crate::frame::Drawable;
use crate::gamesounds::GameSounds;
use crate::drones::Drones;
use crate::player::Player;
use crate::render;

pub fn play(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    init(stdout)?;
    run_game_loop()?;

    exit(stdout)?;
    Ok(())
}

fn run_game_loop() -> Result<(), Box<dyn Error>> {
    // Render loop
    let (render_tx, render_rx) = mpsc::channel(); // should you crossbeam channels instead
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut sounds = GameSounds::new();
    let mut drones = Drones::new();
    let mut player = Player::new();
    let mut instant = Instant::now();
    sounds.startup();
    // game loop
    'gameloop: loop {
        //Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        while event::poll(Duration::default())? {
            // Input
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        sounds.lose();
                        break 'gameloop;
                    }
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            sounds.pew();
                        }
                    }
                    _ => { /*println!("{:?}",key_event.code);*/ }
                }
            }
        }
        // updates
        player.update(delta);
        if drones.update(delta) {
            sounds.march();
        }
        if player.detect_hits(&mut drones) {
            sounds.explode();
        }

        // Draw and Render
        let drawables: Vec<&dyn Drawable> = vec![&player, &drones];
        for d in drawables {
            d.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(2));

        //win or lose
        if drones.all_dead() {
            sounds.win();
            break 'gameloop;
        }

        if drones.reached_bottom() {
            sounds.lose();
            break 'gameloop;
        }
    }
    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    sounds.wait();
    Ok(())
}

fn init(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; // like vim screen
    stdout.execute(Hide)?;
    Ok(())
}

fn exit(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?; // like vim screen
    terminal::disable_raw_mode()?;
    Ok(())
}
