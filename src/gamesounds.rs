
use crate::audio::Audio;

pub struct GameSounds{
    audio: Audio,    
}

impl GameSounds {
    pub fn new() -> Self{
        let mut audio = Audio::new();
        init_audio(&mut audio);
        Self{
            audio,
        }
    }

    pub fn lose(&mut self){
        self.audio.play("lose");
    }
    pub fn win(&mut self){
        self.audio.play("win");
    }
    pub fn pew(&mut self){
        self.audio.play("laser");
    }
    pub fn explode(&mut self){
        self.audio.play("explode");
    }
    pub fn march(&mut self){
        self.audio.play("move");
    }
    pub fn startup(&mut self){
        self.audio.play("startup");
    }

    pub fn wait(&self){
        self.audio.wait();
    }

}

fn init_audio(audio: &mut Audio){
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("laser","laser.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");
    audio.wait();
}