mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
    game.quit();
}
