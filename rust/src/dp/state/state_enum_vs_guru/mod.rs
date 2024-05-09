mod guru;
use guru::player::Player;

pub trait State {
    fn play(&mut self, player: &mut Player);

    fn stop(&mut self, player: &mut Player);

    // Inheritance lmao
    fn next(&self, player: &mut Player) {
        player.next_track();
    }

    fn prev(&self, player: &mut Player) {
        player.prev_track();
    }
}

#[derive(Debug)]
pub enum _State {
    Stopped { data: i32 },
    Pause,
    Play,
}

impl State for _State {
    fn play(&mut self, player: &mut Player) {
        match self {
            _State::Stopped { data } => {
                println!("Player in stopped mode playing now");
                player.play();
                *self = _State::Play
            }
            _State::Pause => {
                println!("Player in paused  mode playing now");
                player.pause();
                *self = _State::Play
            }
            _State::Play => {
                println!("Player in play  mode pausing now");
                player.pause();
                *self = _State::Pause
            }
        }
    }

    fn stop(&mut self, player: &mut Player) {
        match self {
            _State::Stopped { data } => {
                println!("Player in stopped  mode doing nothing");
            }
            _State::Pause => {
                player.pause();
                player.rewind();

                println!("Player in paused state stopping now .");
                *self = _State::Pause
            }
            _State::Play => {
                println!("Player in play mode stopping now ...");
                player.pause();
                player.rewind();

                *self = _State::Stopped { data: 10 }
            }
        }
    }
}

#[cfg(test)]
mod enum_over_state_tests {
    use super::{_State, guru::player::Player, State};
    use std::time::Instant;

    #[test]
    fn test_one() {
        let now = Instant::now();

        let mut player = Player::default();

        let mut state = _State::Stopped { data: 20 };

        for _ in 0..10 {
            state.play(&mut player);
        }

        println!("\n Time elapsed: {:?}", Instant::now().duration_since(now))
    }
}

/*
Player in stopped mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
*/
