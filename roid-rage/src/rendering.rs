use ggez::graphics::{Color, DrawMode, DrawParam, StrokeOptions};
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;
use std::f32::consts::PI;

use crate::components::{Bullet, Fragment, Roid, Transform};

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
