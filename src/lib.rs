use bbggez::ggez::event::EventHandler;
use bbggez::ggez::graphics;
use bbggez::ggez::nalgebra::Point2;
use bbggez::ggez::Context;
use bbggez::ggez::GameResult;
use rand::prelude::*;
use std::collections::HashMap;

pub struct GameOfLife {
    cell_count_row: usize,
    cell_width: f32,
    cell_height: f32,
    grid_line_vertical: graphics::Mesh,
    cell_mesh: graphics::Mesh,
    cell_state_current: Vec<bool>,
    cell_state_next: Vec<bool>,
}

impl GameOfLife {
    pub fn new(context: &mut Context) -> GameOfLife {
        let (arena_width, arena_height) = graphics::drawable_size(context);
        let cell_count_row = 50;
        let cell_width = arena_width / cell_count_row as f32;
        let cell_height = cell_width;
        let grid_line_color = graphics::Color::from_rgb(169, 169, 169);
        let grid_line_vertical =
            bbggez::mesh::create_rect(0.0, 0.0, 2.0, arena_height, grid_line_color, context);
        let cell_count = cell_count_row * cell_count_row;
        let mut cell_state_current = Vec::with_capacity(cell_count);
        let cell_mesh = bbggez::mesh::create_rect(
            0.0,
            0.0,
            cell_width,
            cell_height,
            bbggez::color::random_bright_color(),
            context,
        );
        let mut rng = rand::thread_rng();
        let chance_cell_is_created = 0.2;

        for _ in 0..cell_count {
            let random: f32 = rng.gen();

            if random < chance_cell_is_created {
                cell_state_current.push(true);
            } else {
                cell_state_current.push(false);
            }
        }

        GameOfLife {
            cell_width,
            cell_height,
            cell_count_row,
            grid_line_vertical,
            cell_mesh,
            cell_state_next: cell_state_current.clone(),
            cell_state_current,
        }
    }

    fn count_neighbors(&self, index: usize) -> usize {
        let mut neighbors = 0;
        let is_at_top = index < self.cell_count_row;
        let is_at_left = index.rem_euclid(self.cell_count_row) == 0;
        let is_at_right = index.rem_euclid(self.cell_count_row) == self.cell_count_row - 1;
        let is_at_bottom = index > self.cell_state_current.len() - self.cell_count_row - 1;

        // check upper left
        if !is_at_top && !is_at_left && self.cell_state_current[index - self.cell_count_row - 1] {
            neighbors += 1;
        }
        // check upper
        if !is_at_top && self.cell_state_current[index - self.cell_count_row] {
            neighbors += 1;
        }
        // check upper right
        if !is_at_top && !is_at_right && self.cell_state_current[index - self.cell_count_row + 1] {
            neighbors += 1;
        }
        // check left
        if !is_at_left && self.cell_state_current[index - 1] {
            neighbors += 1;
        }
        // check right
        if !is_at_right && self.cell_state_current[index + 1] {
            neighbors += 1;
        }
        // check upper left
        if !is_at_bottom && !is_at_left && self.cell_state_current[index + self.cell_count_row - 1]
        {
            neighbors += 1;
        }
        // check upper
        if !is_at_bottom && self.cell_state_current[index + self.cell_count_row] {
            neighbors += 1;
        }
        // check upper right
        if !is_at_bottom && !is_at_right && self.cell_state_current[index + self.cell_count_row + 1]
        {
            neighbors += 1;
        }

        neighbors
    }
}

impl EventHandler for GameOfLife {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        // Any live cell with two or three neighbors survives.
        // Any dead cell with three live neighbors becomes a live cell.
        // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
        for (index, &is_alive) in self.cell_state_current.iter().enumerate() {
            // is this a live cell?
            if is_alive {
                // if yes, how many neighbors does it have?
                let neighbor_count = self.count_neighbors(index);
                // if over 3 or under 2, the cell dies
                if neighbor_count < 2 || neighbor_count > 3 {
                    self.cell_state_next[index] = false;
                }
            } else {
                // if no, how many neighbors does it have?
                let neighbor_count = self.count_neighbors(index);
                if neighbor_count == 3 {
                    // if 3, the cell becomes alive
                    self.cell_state_next[index] = true;
                }
            }
        }

        self.cell_state_current = self.cell_state_next.clone();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let (arena_width, arena_height) = graphics::drawable_size(context);

        for width in 0..self.cell_count_row {
            graphics::draw(
                context,
                &self.grid_line_vertical,
                graphics::DrawParam::default()
                    .dest(Point2::new(width as f32 * self.cell_width, 0.0)),
            )?;
            graphics::draw(
                context,
                &self.grid_line_vertical,
                graphics::DrawParam::default()
                    .rotation(-std::f32::consts::FRAC_PI_2)
                    .dest(Point2::new(0.0, width as f32 * self.cell_height)),
            )?;
        }

        graphics::draw(
            context,
            &self.grid_line_vertical,
            graphics::DrawParam::default().dest(Point2::new(arena_width - 2.0, 0.0)),
        )?;
        graphics::draw(
            context,
            &self.grid_line_vertical,
            graphics::DrawParam::default()
                .dest(Point2::new(0.0, arena_height - 1.0))
                .rotation(-std::f32::consts::FRAC_PI_2),
        )?;

        for (index, &is_alive) in self.cell_state_current.iter().enumerate() {
            if is_alive {
                let x = index.rem_euclid(self.cell_count_row);
                let y = index / self.cell_count_row;
                dbg!(9usize / 5usize);
                graphics::draw(
                    context,
                    &self.cell_mesh,
                    graphics::DrawParam::default().dest(Point2::new(
                        x as f32 * self.cell_width,
                        y as f32 * self.cell_height,
                    )),
                )?;
            }
        }

        graphics::present(context)
    }
}

// {
//     "1,1": true,
//     "3,2": true
// }
// [
//     [cell, cell, cell],
//     [cell, {x: 1, y: 1, alive: true}, cell],
//     [cell, cell, cell]
// ]

// [true, false, true]
