use std::time::Instant;

use glam::{Mat3, UVec2, Vec3};
use miniquad::{
    Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, GlContext,
    KeyCode, PassAction, Pipeline, PipelineParams, RenderingBackend, ShaderSource, TextureId,
    VertexAttribute, VertexFormat,
};
use scene::camera::{Camera, Pinhole};
use tracing::pathtracer::Pathtracer;

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
    fn translation(&self) -> Vec3 {
        let f = |(a, b)| match (a, b) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        Vec3::new(f(self.move_x), f(self.move_y), f(self.move_z))
    }
}

pub(crate) struct Stage {
    ctx: Box<GlContext>,

    pipeline: Pipeline,
    bindings: Bindings,
    texture: TextureId,

    worker: Option<Worker>,

    target_size: UVec2,
    camera: Camera,

    last_update: Instant,
    input: InputState,
}

impl Stage {
    pub fn new(pathtracer: Pathtracer, camera: Camera) -> Stage {
        let target_size = UVec2::new(128, 128);
        let pinhole = Pinhole::new(camera.clone(), target_size);
        let worker = Worker::spawn(pathtracer, pinhole);

        let mut ctx = Box::new(GlContext::new());

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
            miniquad::TextureParams {
                format: miniquad::TextureFormat::RGB8,
                width: 0,
                height: 0,
                ..Default::default()
            },
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
            target_size,
            camera,
            last_update: Instant::now(),
            input: InputState::default(),
        }
    }

    fn send_pinhole(&mut self) {
        if let Some(worker) = &self.worker {
            let pinhole = Pinhole::new(self.camera.clone(), self.target_size);
            worker.send(pinhole);
            eprintln!("Sent {:?} at {:?}.", self.target_size, self.camera.position,);
        }
    }

    fn update_input(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_update;
        let translation = self.input.translation();
        if translation != Vec3::ZERO {
            const TRANSLATION_SPEED: f32 = 2.0;
            let distance = delta.as_secs_f32() * TRANSLATION_SPEED;
            let translation_matrix =
                Mat3::from_cols(self.camera.right, self.camera.up, self.camera.direction);
            let position = self.camera.position + translation_matrix * translation * distance;
            self.camera = self.camera.with_position(position);
            self.send_pinhole();
        }
        self.last_update = now;
    }

    fn update_texture(&mut self) {
        while let Some(result) = self.worker.as_ref().and_then(Worker::try_receive) {
            eprintln!(
                "Received {:?} @ {} rendered in {:?}.",
                result.buffer.size, result.iterations, result.duration,
            );
            let texture_size = self.ctx.texture_size(self.texture).into();
            if result.buffer.size == texture_size {
                self.ctx
                    .texture_update(self.texture, &result.buffer.to_rgb8(result.iterations));
            } else {
                self.ctx.delete_texture(self.texture);
                let width = result.buffer.size.x;
                let height = result.buffer.size.y;
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
            }
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
            self.target_size = UVec2::new(width as u32, height as u32);
            self.send_pinhole();
        }
    }

    fn update(&mut self) {
        self.update_input();
        self.update_texture();
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn quit_requested_event(&mut self) {
        eprintln!("Exiting...");
        let worker = self.worker.take().unwrap();
        worker.join();
    }
}

mod shader {
    use miniquad::{ShaderMeta, UniformBlockLayout};

    pub const VERTEX: &str = r"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        texcoord = in_uv;
    }";

    pub const FRAGMENT: &str = r"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }";

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
