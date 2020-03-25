use ggez::graphics::{Color, DrawMode, DrawParam, StrokeOptions};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use std::f32::consts::PI;

use crate::components::{Bullet, Fragment, Roid, Transform};

pub trait Render {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()>;
}

impl Render for Roid {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()> {
        let angle_step = (PI * 2.0) / self.points.len() as f32;
        let center = Point2::<f32>::new(
            transform.0.translation.vector.x,
            transform.0.translation.vector.y,
        );
        let line_points: Vec<Point2<f32>> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let angle = angle_step * i as f32;
                let offset = Vector2::<f32>::new(angle.cos(), angle.sin()) * *p;
                center + offset
            })
            .collect();

        let mb = &mut graphics::MeshBuilder::new();
        mb.polygon(
            DrawMode::Stroke(StrokeOptions::DEFAULT),
            &line_points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}

impl Render for Bullet {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            Point2::<f32>::new(
                transform.0.translation.vector.x,
                transform.0.translation.vector.y,
            ),
            Bullet::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}

impl Render for Fragment {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            Point2::<f32>::new(
                transform.0.translation.vector.x,
                transform.0.translation.vector.y,
            ),
            Fragment::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}
