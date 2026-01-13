use ggez::glam::Vec2 as GgezVec2;
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, StrokeOptions};
use ggez::{Context, GameResult};
use glam::Vec2 as GameVec2;
use std::f32::consts::PI;

type Point2 = GgezVec2;

use crate::components::{Bullet, Fragment, Roid, Ship};

pub trait Render {
    fn render(
        &self,
        position: GameVec2,
        direction: f32,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> GameResult<()>;
}

impl Render for Roid {
    fn render(
        &self,
        position: GameVec2,
        direction: f32,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        let angle_step = (PI * 2.0) / self.points.len() as f32;
        let line_points: Vec<Point2> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, distance)| {
                let angle = angle_step * i as f32;
                Point2::new(angle.cos(), angle.sin()) * *distance
            })
            .collect();

        let mb = &mut graphics::MeshBuilder::new();
        mb.polygon(
            DrawMode::Stroke(StrokeOptions::DEFAULT),
            &line_points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let mesh = graphics::Mesh::from_data(ctx, mb.build());
        let param = DrawParam::new()
            .rotation(direction)
            .dest(to_ggez_vec2(position));
        canvas.draw(&mesh, param);
        Ok(())
    }
}

impl Render for Bullet {
    fn render(
        &self,
        position: GameVec2,
        _direction: f32,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            to_ggez_vec2(position),
            Bullet::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mesh = graphics::Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        Ok(())
    }
}

impl Render for Fragment {
    fn render(
        &self,
        position: GameVec2,
        _direction: f32,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            to_ggez_vec2(position),
            Fragment::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mesh = graphics::Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        Ok(())
    }
}

impl Render for Ship {
    fn render(
        &self,
        position: GameVec2,
        direction: f32,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        // let center = Point2::new(0.0, 0.0);
        let points = vec![
            Point2::new(self.length / 2.0, 0.0),
            Point2::new(-1.0 * self.length / 2.0, -1.0 * self.width / 2.0),
            Point2::new(-1.0 * self.length / 2.0, self.width / 2.0),
        ];

        mb.polygon(
            DrawMode::stroke(1.0),
            &points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mesh = graphics::Mesh::from_data(ctx, mb.build());
        let param = DrawParam::new()
            .rotation(direction)
            .dest(to_ggez_vec2(position));
        canvas.draw(&mesh, param);
        Ok(())
    }
}

fn to_ggez_vec2(value: GameVec2) -> GgezVec2 {
    GgezVec2::new(value.x, value.y)
}
