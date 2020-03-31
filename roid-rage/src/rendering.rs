use ggez::graphics::{Color, DrawMode, DrawParam, StrokeOptions};
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;
use std::f32::consts::PI;

use crate::components::{Bullet, Fragment, Roid, Ship, Transform};

pub trait Render {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()>;
}

impl Render for Roid {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()> {
        let angle_step = (PI * 2.0) / self.points.len() as f32;
        let line_points: Vec<ggez::nalgebra::Point2<f32>> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, distance)| {
                let angle = angle_step * i as f32;
                let edge_point = Point2::<f32>::new(angle.cos(), angle.sin()) * *distance;
                let transformed = transform.0.transform_point(&edge_point);
                ggez::nalgebra::Point2::new(transformed.x, transformed.y)
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
            ggez::nalgebra::Point2::<f32>::new(
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
            ggez::nalgebra::Point2::<f32>::new(
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

impl Render for Ship {
    fn render(&self, transform: &Transform, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        let center = ggez::nalgebra::Point2::<f32>::new(
            transform.0.translation.vector.x,
            transform.0.translation.vector.y,
        );
        let points = vec![
            center + ggez::nalgebra::Vector2::<f32>::new(self.length / 2.0, 0.0),
            center
                + ggez::nalgebra::Vector2::<f32>::new(
                    -1.0 * self.length / 2.0,
                    -1.0 * self.width / 2.0,
                ),
            center
                + ggez::nalgebra::Vector2::<f32>::new(-1.0 * self.length / 2.0, self.width / 2.0),
        ];
        mb.polygon(
            DrawMode::stroke(1.0),
            &points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}
