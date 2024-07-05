use std::time::Instant;

use miniquad::{
    window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode,
    Pipeline, PipelineParams, RenderingBackend, ShaderSource, TextureId, VertexAttribute,
    VertexFormat,
};
use nalgebra::{Matrix3, Vector3};
use scene::camera::{Camera, Pinhole};

use crate::worker::Worker;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

#[derive(Default)]
struct InputState {
    move_x: (bool, bool),
    move_y: (bool, bool),
    move_z: (bool, bool),
}

impl InputState {
    fn translation(&self) -> Vector3<f32> {
        let f = |(a, b)| match (a, b) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        Vector3::new(f(self.move_x), f(self.move_y), f(self.move_z))
    }
}

pub(crate) struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,
    texture: TextureId,

    worker: Option<Worker>,

    camera: Camera,

    last_update: Instant,
    input: InputState,
}

impl Stage {
    pub fn new(worker: Worker, camera: Camera) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let full_screen_quad: [Vertex; 4] = [
            Vertex { pos: Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 1. } },
            Vertex { pos: Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos: Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos: Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 0. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&full_screen_quad),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let texture = ctx.new_texture(
            miniquad::TextureAccess::Static,
            miniquad::TextureSource::Empty,
            Default::default(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            ctx,
            pipeline,
            bindings,
            texture,
            worker: Some(worker),
            camera,
            last_update: Instant::now(),
            input: InputState::default(),
        }
    }
}

impl EventHandler for Stage {
    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::A => self.input.move_x.0 = true,
            KeyCode::E => self.input.move_x.1 = true,
            KeyCode::Semicolon => self.input.move_y.0 = true,
            KeyCode::Period => self.input.move_y.1 = true,
            KeyCode::O => self.input.move_z.0 = true,
            KeyCode::Comma => self.input.move_z.1 = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, _keymods: miniquad::KeyMods) {
        match keycode {
            KeyCode::A => self.input.move_x.0 = false,
            KeyCode::E => self.input.move_x.1 = false,
            KeyCode::Semicolon => self.input.move_y.0 = false,
            KeyCode::Period => self.input.move_y.1 = false,
            KeyCode::O => self.input.move_z.0 = false,
            KeyCode::Comma => self.input.move_z.1 = false,
            _ => (),
        }
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        if !cfg!(debug_assertions) {
            if let Some(worker) = &self.worker {
                let pinhole = Pinhole::new(self.camera.clone(), width as u32, height as u32);
                worker.send(pinhole);
            }
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_update).as_secs_f32();
        if let Some(worker) = &self.worker {
            let translation = self.input.translation();
            if translation != Vector3::zeros() {
                const TRANSLATION_SPEED: f32 = 2.0;
                let distance = delta * TRANSLATION_SPEED;
                let translation_matrix = Matrix3::from_rows(&[
                    self.camera.right.into_inner().transpose(),
                    self.camera.up.into_inner().transpose(),
                    self.camera.direction.into_inner().transpose(),
                ]);
                let position = self.camera.position + translation_matrix * translation * distance;
                self.camera = self.camera.with_position(position);
                let texture_size = self.ctx.texture_size(self.texture);
                let pinhole = Pinhole::new(self.camera.clone(), texture_size.0, texture_size.1);
                worker.send(pinhole);
            }

            if let Some(result) = worker.try_receive() {
                eprintln!(
                    "Received {:?} @ {} rendered in {:?}.",
                    [result.buffer.width, result.buffer.height],
                    result.iterations,
                    result.duration,
                );
                let texture_size = self.ctx.texture_size(self.texture);
                if result.buffer.size() != texture_size {
                    self.ctx.delete_texture(self.texture);
                    let width = result.buffer.width;
                    let height = result.buffer.height;
                    self.texture = self.ctx.new_texture_from_data_and_format(
                        &result.buffer.to_rgb8(result.iterations),
                        miniquad::TextureParams {
                            format: miniquad::TextureFormat::RGB8,
                            width,
                            height,
                            ..Default::default()
                        },
                    );
                    self.bindings.images = vec![self.texture];
                } else {
                    self.ctx
                        .texture_update(self.texture, &result.buffer.to_rgb8(result.iterations))
                }
            }
        }
        self.last_update = now;
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn quit_requested_event(&mut self) {
        eprintln!("Exiting");
        let worker = self.worker.take().unwrap();
        worker.join();
    }
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
