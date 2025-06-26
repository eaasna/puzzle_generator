pub mod draw{

pub use miniquad::*;

const N: usize = 5;
const T: usize = (N-1)*(N-1)*2*3; // 2 triangles per square, 3 indices per triangle

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
}


impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        
        let vertices: [Vertex; 2*N*N] = core::array::from_fn(|i| {
            if i < N*N {
                Vertex {
                    pos : [-1. + 2.*((i%N) as f32)/(N as f32 -1.),  // x
                           -1. + 2.*((i/N) as f32)/(N as f32 -1.) ], // y
                    color: [0., 0., 1., 0.]
                }
            } else {
                Vertex {
                    pos : [-1. + 2.*(((i-N*N)%N) as f32)/(N as f32 -1.),  // x
                           -1. + 2.*(((i-N*N)/N) as f32)/(N as f32 -1.)], // y
                    color: [1., 0., 0., 1.]
                }
        }
        });

        print!("BLUE\n");

        for i in 0..N*N {
            print!("Vertex {}: {},{}\n", i, vertices[i].pos[0], vertices[i].pos[1]);
        }

        print!("RED\n");

        for i in N*N..2*N*N {
            print!("Vertex {}: {},{}\n", i, vertices[i].pos[0], vertices[i].pos[1]);
        }
        
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let mut indices: [u16; 2*T] = [0; 2*T];

        let mut add = 0;
        let s: [usize; 3*2] = [0, 1, N, N, (N+1), 1];
        for i in 0..T {
            if (i % ((N-1)*3*2) == 0) && (i != 0) {
                add += 1;
            }
            indices[i] = (s[i % (N+1)] + i/(3*2) + add) as u16;
        }

        print!("BLUE\n");

        for i in (0..T).step_by(3) {
            print!("Index {}: {},{},{}\n", i, indices[i], indices[i+1], indices[i+2]);
        }

        add = 0;
        for i in 0..T as usize {
            /*if i % 12 < 6 {
                add += 1;
            }*/
            if (i % ((N-1)*3*2) == 0) && (i != 0) {
                add += 1;
            }
            indices[i+T] = (s[i % (N+1)] + i/(3*2) + N*N + add) as u16;
            if (i % 12 < 6) && (add % 2 == 0) {
                indices[i+T] += 1;
            }
        }

        print!("RED\n");

        for i in (T..T*2-2).step_by(3) {
            print!("Index {}: {},{},{}\n", i, indices[i], indices[i+1], indices[i+2]);
        }

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let t = date::now();
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        //for i in 0..(T as f32/6 as f32) as usize {
        for i in 0..2 as usize {
            let t = t + i as f64 * 0.3;

            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                }));
            self.ctx.draw(0, (T*2).try_into().unwrap(), 1);
        }

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

pub fn draw_window() {
    let mut conf = conf::Conf::default();
    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec4 in_color;

    varying lowp vec4 color;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        color = in_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Uniforms
    {
        float2 offset;
    };

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float4 in_color [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy, 0.0, 1.0);
        out.color = v.in_color;

        return out;
    }

    vertex RasterizerData vertexShader(
      Vertex v [[stage_in]], 
      constant Uniforms& uniforms [[buffer(0)]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy + uniforms.offset, 0.0, 1.0);
        out.uv = v.in_uv;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return in.color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}

}
