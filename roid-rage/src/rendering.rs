use ggez::graphics::{Color, DrawMode, DrawParam, StrokeOptions};
use ggez::nalgebra as gn;
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;
use std::f32::consts::PI;

use crate::components::{Bullet, Fragment, Roid, Ship};

pub trait Render {
    fn render(&self, position: Point2<f32>, rotation: f32, ctx: &mut Context) -> GameResult<()>;
}

impl Render for Roid {
    fn render(&self, position: Point2<f32>, rotation: f32, ctx: &mut Context) -> GameResult<()> {
        let angle_step = (PI * 2.0) / self.points.len() as f32;
        let line_points: Vec<gn::Point2<f32>> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, distance)| {
                let angle = angle_step * i as f32;
                gn::Point2::<f32>::new(angle.cos(), angle.sin()) * *distance
            })
            .collect();

        let mb = &mut graphics::MeshBuilder::new();
        mb.polygon(
            DrawMode::Stroke(StrokeOptions::DEFAULT),
            &line_points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let mesh = mb.build(ctx)?;
        let param = DrawParam::new()
            .rotation(rotation)
            .dest(gn::Point2::new(position.x, position.y));
        graphics::draw(ctx, &mesh, param)
    }
}

impl Render for Bullet {
    fn render(&self, position: Point2<f32>, _rotation: f32, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            gn::Point2::<f32>::new(position.x, position.y),
            Bullet::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}

impl Render for Fragment {
    fn render(&self, position: Point2<f32>, _rotation: f32, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.circle(
            DrawMode::fill(),
            gn::Point2::<f32>::new(position.x, position.y),
            Fragment::radius(),
            0.1,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new())
    }
}

impl Render for Ship {
    fn render(&self, position: Point2<f32>, rotation: f32, ctx: &mut Context) -> GameResult<()> {
        let mb = &mut graphics::MeshBuilder::new();
        // let center = Point2::<f32>::new(0.0, 0.0);
        let points = vec![
            gn::Point2::<f32>::new(self.length / 2.0, 0.0),
            gn::Point2::<f32>::new(-1.0 * self.length / 2.0, -1.0 * self.width / 2.0),
            gn::Point2::<f32>::new(-1.0 * self.length / 2.0, self.width / 2.0),
        ];

        mb.polygon(
            DrawMode::stroke(1.0),
            &points,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mesh = mb.build(ctx)?;
        let param = DrawParam::new()
            .rotation(rotation)
            .dest(gn::Point2::new(position.x, position.y));
        graphics::draw(ctx, &mesh, param)
    }
}
