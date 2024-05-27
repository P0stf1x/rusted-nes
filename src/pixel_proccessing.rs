use std::num::Wrapping;

use crate::memory::*;

use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub struct PPU {
    pub memory: MEM,
}

impl PPU {
    pub fn run(&mut self) {
        use std::env;
        // env::set_var("LIBRARY_PATH", "C:\\Coding\\rust\\lib");
        // env::set_var("LIB", "C:\\Coding\\rust\\lib");
        // use glium::Surface;
        // use glutin::{
        //     config::ConfigTemplateBuilder,
        //     context::{ContextAttributesBuilder, NotCurrentGlContext},
        //     display::{GetGlDisplay, GlDisplay},
        //     surface::{SurfaceAttributesBuilder, WindowSurface},
        // };
        // use imgui_sdl2_support::SdlPlatform;
    
        /* initialize SDL and its video subsystem */
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
    
        /* hint SDL to initialize an OpenGL 3.3 core profile context */
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);
    
        /* create a new window, be sure to call opengl method on the builder when using glow! */
        let window = video_subsystem
            .window("Hello imgui-rs!", 1280, 720)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
    
        /* create a new OpenGL context and make it current */
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();
    
        /* enable vsync to cap framerate */
        window.subsystem().gl_set_swap_interval(1).unwrap();
    
        /* create new glow and imgui contexts */
        let mut gl = glow_context(&window);
    
        /* create context */
        let mut imgui = Context::create();
    
        /* disable creation of files on disc */
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);
    
        /* setup platform and renderer, and fonts to imgui */
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
    
        /* create textures, platform and renderer */
        let mut textures = imgui::Textures::<glow::Texture>::default();
        let mut platform = SdlPlatform::init(&mut imgui);
        let mut renderer = Renderer::initialize(&mut gl, &mut imgui, &mut textures, true).unwrap();
    
        /* start main loop */
        let mut event_pump = sdl.event_pump().unwrap();
    
        // drawing sample data
        let mut image_data = Vec::with_capacity(256 * 256);
        for y in 0..256 {
            for x in 0..256 {
                // Insert RGB values
                image_data.push(x as u8);
                image_data.push(y as u8);
                image_data.push((x + y) as u8);
            }
        }
    
        // creating image
        let gl_image;
        unsafe {
            gl_image = gl.create_texture().ok();
            gl.bind_texture(glow::TEXTURE_2D, gl_image);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGB as i32, 256 as i32, 256 as i32, 0, glow::RGB, glow::UNSIGNED_BYTE, Some(&image_data));
        }
        let sample_texture = textures.insert(gl_image.unwrap());
    
        // reading memory
        let mut memory_bytes: String;
        unsafe {
            memory_bytes = String::from_utf8_unchecked(self.memory.data.escape_ascii().collect());
        }
        let mem_byts = &memory_bytes[0x8000*4..0x8005*4];
    
        'main: loop {
            for event in event_pump.poll_iter() {
                /* pass all events to imgui platfrom */
                platform.handle_event(&mut imgui, &event);
    
                if let Event::Quit { .. } = event {
                    break 'main;
                }
            }
    
            /* call prepare_frame before calling imgui.new_frame() */
            platform.prepare_frame(&mut imgui, &window, &event_pump);
    
            let ui = imgui.new_frame();
            /* create imgui UI here */
            ui.show_demo_window(&mut true);
    
            ui.window("rendering test")
                .size([300.0, 400.0], Condition::FirstUseEver)
                .build(|| {
                    // ui.input_text_multiline("memory", mem_byts, [32.0, 32.0]);
                    ui.text_wrapped(&mem_byts);
                    Image::new(sample_texture, [256.0, 256.0]).build(&ui);
                    // textures_ui.show(ui);
                    // ui.text_wrapped(&readme);
                    // ui.clipboard_text();
                    ui.text_wrapped("end");
                });
    
            /* render */
            let draw_data = imgui.render();
    
            unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };
            renderer.render(&mut gl, &textures, draw_data).unwrap();
    
            window.gl_swap_window();
        }
    }
}
