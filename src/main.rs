use std::num::Wrapping;

use crate::processor::*;
use crate::memory::*;

mod processor;
mod memory;

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut memory;
    if args.len() > 1 {
        memory = MEM::new_from(&args[1]);
    } else {
        memory = MEM::new(MEMORY_SIZE);
    }
    let mut cpu: CPU = Default::default();

    // TODO: move to cpu init
    cpu.PC = std::num::Wrapping(PRG_ROM_ENTRY_ADDR as u16);
    cpu.S = Wrapping(0xFDu8);
    cpu.I = true;
    memory.data[0x2002] = 0b_1000_0000; // FIXME: hack to make cpu think it's always in vblank
    
    // use std::io::Write;
    // use std::fs;
    // let file = fs::OpenOptions::new()
    //     .create(true) // To create a new file
    //     .write(true)
    //     // either use the ? operator or unwrap since it returns a Result
    //     .open("initial_memory.dump");

    // let _ = match file {
    //     Ok(mut f) => f.write_all(&memory.data),
    //     Err(_) => Ok(println!("No file")),
    // };
    
    // loop {
    //     if cpu.execute(&mut memory).is_err() {
    //         // TODO: use logger instead
    //         println!("");
    //         println!("-----------------------------");
    //         println!("WE CRASHED");
    //         println!("{:#04X?}", cpu);
    //         println!("{:#04X}", memory.read(cpu.PC.0 as usize, 1));
    //         println!("-----------------------------");
    //         panic!();
    //     };
    // }

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
    let gl = glow_context(&window);

    /* create context */
    let mut imgui = Context::create();

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    /* create platform and renderer */
    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

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

        /* render */
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }
}
