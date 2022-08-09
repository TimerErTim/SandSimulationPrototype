use tetra::{Context, Event, graphics, State, TetraError};
use tetra::graphics::Color;

const SIMULATION_HEIGHT: usize = 500;
const SIMULATION_WIDTH: usize = 1000;

trait CellBehaviour {
    const COLOR: Color;
}

struct EmptyCell;

impl CellBehaviour for EmptyCell {
    const COLOR: Color = Color::rgba(0., 0., 0., 0.);
}

struct Cell {
    updated: bool,
    behaviour: Box<dyn CellBehaviour>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            updated: false,
            behaviour: Box::new(EmptyCell),
        }
    }
}

struct GameState {
    world: [[Cell; SIMULATION_HEIGHT]; SIMULATION_WIDTH],
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            world: [[0; SIMULATION_HEIGHT]; SIMULATION_WIDTH].map(|row| row.map(|_| Cell::default()))
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::WHITE);

        for column in 0..self.world.len() {
            for row in 0..self.world[column].len() {
                // TODO: Draw tile
            }
        }

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        Ok(())
    }
}

fn main() -> tetra::Result {
    tetra::ContextBuilder::new("This is real shit", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
