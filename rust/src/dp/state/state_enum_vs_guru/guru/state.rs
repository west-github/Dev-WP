use super::{player::Player, State};

#[derive(Clone, Debug)]
pub struct StoppedState;
#[derive(Clone, Debug)]
pub struct PausedState;
#[derive(Clone, Debug)]
pub struct PlayingState;

impl State for StoppedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        println!("Player in stopped  mode playing now");

        player.play();

        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        println!("Player in stopped  mode doing nothing");
        self
    }
}

impl State for PausedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        println!("Player in paused  mode playing now");
        player.pause();

        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.pause();
        player.rewind();

        println!("Player in paused state stopping now");

        Box::new(StoppedState)
    }
}

impl State for PlayingState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        println!("Player in play  mode pausing now");
        player.pause();

        Box::new(PausedState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        println!("Player in play mode stopping  now");
        player.pause();
        player.rewind();
        Box::new(StoppedState)
    }
}
