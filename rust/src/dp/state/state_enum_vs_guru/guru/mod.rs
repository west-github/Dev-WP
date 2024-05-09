use self::player::Player;
pub mod player;
mod state;

pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
}

impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.next_track();

        self
    }

    pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        player.prev_track();

        self
    }
}

#[cfg(test)]
mod state_over_enum_tests {
    use std::time::Instant;

    use super::{player::Player, state::StoppedState, State};

    #[test]
    fn test_one() {
        let now = Instant::now();

        let mut player = Player::default();
        let mut state: Box<dyn State> = Box::new(StoppedState);

        for _ in 0..10 {
            state = state.play(&mut player);
        }

        println!("\n Time elapsed: {:?}", Instant::now().duration_since(now))
    }
}
/*

Player in stopped  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
Player in paused  mode playing now
Player in play  mode pausing now
*/
