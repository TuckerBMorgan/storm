use bounded_spsc_queue::Consumer;
use cgmath::*;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::message::RenderFrame;
use render::shader::shape::*;
use render::vertex::shape::*;
use time::frame_clock::*;

pub struct RenderConsumer {
    display: Display,
    clock: FrameClock,
    frame_consumer: Consumer<RenderFrame>,
    shape_shader: ShapeShader,
    triangle_buffer: GeometryBuffer<Triangle<ShapeVertex>>,
    quad_buffer: GeometryBuffer<Quad<ShapeVertex>>,
}

impl RenderConsumer {
    pub fn new(display: Display, frame_consumer: Consumer<RenderFrame>) -> RenderConsumer {
        // Get the composed consumer
        let mut consumer = RenderConsumer {
            display: display,
            clock: FrameClock::new(),
            frame_consumer: frame_consumer,
            shape_shader: ShapeShader::new(),
            triangle_buffer: Triangle::new_geometry_buffer(),
            quad_buffer: Quad::new_geometry_buffer(),
        };
        // Initialize it
        consumer.initialize();
        // Return
        consumer
    }

    fn initialize(&mut self) {
        self.display.enable_clear_color();
        self.display.clear_color(0.0, 0.0, 0.0, 1.0);
        self.clock.set_fps(20000);
        self.shape_shader.bind();
        self.shape_shader
            .set_translation(Vector3::new(0f32, 0f32, 0f32));
        println!(
            "Render: OpenGL version {}",
            self.display.get_version_string()
        );
    }

    pub fn handle_create_quad(&mut self, messages: &mut Vec<CreateQuadMessage>) {
        for message in messages.drain(..) {
            self.quad_buffer.add(message.quad);
        }
    }

    pub fn handle_create_triangle(&mut self, messages: &mut Vec<CreateTriangleMessage>) {
        for message in messages.drain(..) {
            self.triangle_buffer.add(message.triangle);
        }
    }

    pub fn tick(&mut self) {
        // Frame processing
        match self.frame_consumer.try_pop().as_mut() {
            Some(f) => {
                // Message processing
                self.handle_create_quad(&mut f.create_quad);
                self.handle_create_triangle(&mut f.create_triangle);

                // Sync buffers
                self.quad_buffer.sync();
                self.triangle_buffer.sync();
            },
            None => {},
        }

        // Shapes
        self.shape_shader.bind();
        self.quad_buffer.draw();
        self.triangle_buffer.draw();

        // Finish
        self.display.swap_buffers();
        self.display.clear();

        // Timing
        self.clock.tick();
    }
}
