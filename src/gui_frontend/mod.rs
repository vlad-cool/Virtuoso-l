use std::borrow::Cow;
use std::cell::RefCell;
use std::process::{Command, Output};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::MutexGuard;
use std::time::Duration;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::gui_frontend::layout_structure::Layout;
use crate::hw_config::Resolution;
use crate::match_info::MatchInfo;
use crate::modules::VirtuosoModule;
use crate::modules::VirtuosoModuleContext;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

// mod colors;
mod layout_structure;
mod layouts;
// mod widgets;

// #[path = "main_screen/cyrano/cyrano_status.rs"]
// mod cyrano_status;

// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/competition_type.rs"]
// mod competition_type;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/cyrano_state.rs"]
// mod cyrano_state;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_id.rs"]
// mod fencer_id;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_medical.rs"]
// mod fencer_medical;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_name.rs"]
// mod fencer_name;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_nation.rs"]
// #[cfg(feature = "cyrano_server")]
// mod fencer_nation;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_reserve.rs"]
// mod fencer_reserve;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/fencer_status.rs"]
// mod fencer_status;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/phase.rs"]
// mod phase;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/piste.rs"]
// mod piste;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/poul_tab.rs"]
// mod poul_tab;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/referee_name.rs"]
// mod referee_name;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/referee_nation.rs"]
// mod referee_nation;
// #[cfg(feature = "cyrano_server")]
// #[path = "main_screen/cyrano/start_time.rs"]
// mod start_time;

// #[path = "main_screen/auto_status.rs"]
// mod auto_status;
// #[path = "main_screen/led_repeater.rs"]
// mod led_repeater;
// #[path = "settings_menu/menu.rs"]
// mod menu;
// #[path = "main_screen/message.rs"]
// mod message;
// #[path = "main_screen/passive_card.rs"]
// mod passive_card;
// #[path = "main_screen/passive_counter.rs"]
// mod passive_counter;
// #[path = "main_screen/passive_indicator.rs"]
// mod passive_indicator;
// #[path = "main_screen/penalty_card.rs"]
// mod penalty_card;
// #[path = "main_screen/period.rs"]
// mod period;
// #[path = "main_screen/priority.rs"]
// mod priority;
// #[path = "main_screen/score.rs"]
// mod score;
// #[path = "main_screen/timer.rs"]
// mod timer;
// #[path = "main_screen/weapon.rs"]
// mod weapon;

// #[derive(Clone)]
// struct WidgetContext<'a> {
//     pub canvas: Rc<RefCell<Canvas<Window>>>,
//     pub texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
//     pub ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
//     pub font_bytes: &'a [u8],
//     pub layout: &'a Layout,
//     pub logger: &'a Logger,
// }

// impl<'a> WidgetContext<'a> {
//     pub fn get_font(&self, font_size: u16) -> Rc<sdl2::ttf::Font<'a, 'a>> {
//         let rwops: RWops<'_> = RWops::from_bytes(self.font_bytes).unwrap_with_logger(self.logger);
//         let font: sdl2::ttf::Font<'a, 'a> = self
//             .ttf_context
//             .load_font_from_rwops(rwops, font_size)
//             .unwrap_with_logger(self.logger);
//         Rc::new(font)
//     }
// }

// trait VirtuosoWidget {
//     fn update(&mut self, data: &MatchInfo);
//     fn render(&mut self);
// }

pub struct GuiFrontend {
    context: VirtuosoModuleContext,
    layout: layout_structure::Layout,
}

impl GuiFrontend {
    pub fn new(context: VirtuosoModuleContext) -> Self {
        let layout: layout_structure::Layout = match context.hw_config.display.resolution {
            Resolution::Res1920X1080 => layouts::LAYOUT_1920X1080,
            Resolution::Res1920X550 => layouts::LAYOUT_1920X550,
            Resolution::Res1920X480 => layouts::LAYOUT_1920X480,
            Resolution::Res1920X360 => layouts::LAYOUT_1920X360,
        };

        Self { context, layout }
    }

    #[cfg(feature = "embeded_device")]
    fn setup_wayland(&self, resolution: Resolution) {
        let output: Result<Output, std::io::Error> = match self.context.hw_config.display.resolution
        {
            Resolution::Res1920X1080 => Command::new("wlr-randr")
                .arg("--output")
                .arg("HDMI-A-1")
                .arg("--custom-mode")
                .arg("1920x1080@60")
                .output(),
            Resolution::Res1920X550 => Command::new("wlr-randr")
                .arg("--output")
                .arg("HDMI-A-1")
                .arg("--custom-mode")
                .arg("1920x550@60")
                .output(),
            Resolution::Res1920X480 => Command::new("wlr-randr")
                .arg("--output")
                .arg("HDMI-A-1")
                .arg("--custom-mode")
                .arg("1920x480@60")
                .arg("--transform")
                .arg("--90")
                .output(),
            Resolution::Res1920X360 => Command::new("wlr-randr")
                .arg("--output")
                .arg("HDMI-A-1")
                .arg("--custom-mode")
                .arg("1920x360@60")
                .output(),
        };

        match output {
            Ok(output) => {
                let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
                let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

                if !stdout.trim().is_empty() {
                    self.context.logger.error(format!(
                        "{} {}",
                        "Warning: wlr-randr stdout is not empty, stdout:", stdout
                    ));
                }

                if !stderr.trim().is_empty() {
                    self.context.logger.error(format!(
                        "{} {}",
                        "Warning: wlr-randr stderr is not empty, stdout:", stdout
                    ));
                }
            }
            Err(err) => {
                self.context
                    .logger
                    .error(format!("Failed to run wlr-randr, err: {err}"));
            }
        }
    }
}

impl VirtuosoModule for GuiFrontend {
    fn run(self) {
        #[cfg(feature = "embeded_device")]
        self.setup_wayland(self.context.display.resolution);

        // let sdl_context: sdl2::Sdl = match sdl2::init() {
        //     Ok(sdl_context) => sdl_context,
        //     Err(err) => {
        //         self.context
        //             .logger
        //             .critical_error(format!("Failed to init sdl, error: {err}"));
        //         return;
        //     }
        // };

        // let video_subsystem: sdl2::VideoSubsystem = match sdl_context.video() {
        //     Ok(video_subsystem) => video_subsystem,
        //     Err(err) => {
        //         self.context
        //             .logger
        //             .critical_error(format!("Failed to create video subsystem, error: {err}"));
        //         return;
        //     }
        // };

        // let ttf_context: sdl2::ttf::Sdl2TtfContext = match sdl2::ttf::init() {
        //     Ok(ttf_context) => ttf_context,
        //     Err(err) => {
        //         self.context
        //             .logger
        //             .critical_error(format!("Failed to init ttf context, error: {err}"));
        //         return;
        //     }
        // };

        // let window: Window = match video_subsystem
        //     .window(
        //         "Virtuoso",
        //         self.layout.background.width as u32,
        //         self.layout.background.height as u32,
        //     )
        //     .build()
        // {
        //     Ok(window) => window,
        //     Err(err) => {
        //         self.context
        //             .logger
        //             .critical_error(format!("Failed to create window, error: {err}"));
        //         return;
        //     }
        // };

        // let canvas = match window.into_canvas().build() {
        //     Ok(canvas) => canvas,
        //     Err(err) => {
        //         self.context
        //             .logger
        //             .critical_error(format!("Failed to create canvas, error: {err}"));
        //         return;
        //     }
        // };

        // let canvas: Rc<RefCell<Canvas<Window>>> = Rc::new(RefCell::new(canvas));

        // canvas.borrow_mut().set_draw_color(colors::BACKGROUND);

        // let font_bytes: &[u8] = include_bytes!("../../assets/AGENCYB.ttf") as &[u8];

        // let texture_creator: sdl2::render::TextureCreator<WindowContext> =
        //     canvas.borrow().texture_creator();

        // let widget_context: WidgetContext<'_> = WidgetContext {
        //     canvas: canvas.clone(),
        //     texture_creator: &texture_creator,
        //     ttf_context: &ttf_context,
        //     font_bytes,
        //     layout: &self.layout,
        //     logger: &self.context.logger,
        // };

        // canvas.borrow_mut().clear();

        // let mut widgets: Vec<Box<dyn VirtuosoWidget>> = Vec::<Box<dyn VirtuosoWidget>>::new();

        // widgets.push(Box::new(auto_status::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(led_repeater::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(message::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(passive_card::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(passive_counter::Drawer::new(
        //     widget_context.clone(),
        // )));
        // widgets.push(Box::new(passive_indicator::Drawer::new(
        //     widget_context.clone(),
        // )));
        // widgets.push(Box::new(penalty_card::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(period::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(priority::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(score::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(timer::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(weapon::Drawer::new(widget_context.clone())));

        // // widgets.push(Box::new(fencer_name::Drawer::new(widget_context.clone())));
        // // widgets.push(Box::new(fencer_nation::Drawer::new(widget_context.clone())));
        // // widgets.push(Box::new(cyrano_status::Drawer::new(widget_context.clone())));

        // #[cfg(feature = "cyrano_server")]
        // {
        //     if let Some(widget) = referee_name::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = referee_nation::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_name::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_id::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_nation::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_status::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_medical::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = fencer_reserve::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = piste::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = phase::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = competition_type::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = poul_tab::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = start_time::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        //     if let Some(widget) = cyrano_state::Drawer::new(widget_context.clone()) {
        //         widgets.push(Box::new(widget));
        //     }
        // }

        // let mut settings_menu: menu::Drawer<'_> = menu::Drawer::new(widget_context.clone());

        // canvas.borrow_mut().present();

        // let mut event_pump: sdl2::EventPump = sdl_context
        //     .event_pump()
        //     .unwrap_with_logger(&self.context.logger);

        // 'running: loop {
        //     std::thread::sleep(Duration::from_millis(20));

        //     for event in event_pump.poll_iter() {
        //         match event {
        //             sdl2::event::Event::Quit { .. } => break 'running,
        //             _ => {}
        //         }
        //     }

        //     if self
        //         .context
        //         .settings_menu_shown
        //         .load(std::sync::atomic::Ordering::Relaxed)
        //     {
        //         canvas.borrow_mut().clear();
        //         settings_menu.update(&self.context.settings_menu.lock().unwrap());
        //         settings_menu.render();
        //         canvas.borrow_mut().present();
        //         std::thread::sleep(Duration::from_millis(40));
        //     } else {
        //         let data: MutexGuard<'_, MatchInfo> = self
        //             .context
        //             .match_info
        //             .lock()
        //             .unwrap_with_logger(&self.context.logger);
        //         for widget in &mut widgets {
        //             widget.update(&data);
        //         }
        //         std::mem::drop(data);

        //         canvas.borrow_mut().clear();
        //         for widget in &mut widgets {
        //             widget.render();
        //         }
        //         canvas.borrow_mut().present();
        //     }
        // }

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app: App = App::default();
        event_loop.run_app(&mut app).unwrap();
    }
}

use glyphon::{
    Attrs, Buffer, BufferLine, FontSystem, Metrics, SwashCache, TextAtlas, TextRenderer,
};

use wgpu::util::DeviceExt;

struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    num_vertices: u32,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let num_vertices = VERTICES.len() as u32;

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                // buffers: &[],                 // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),

                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            render_pipeline,
            vertex_buffer,
            num_vertices,
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    fn render(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder: wgpu::CommandEncoder =
            self.device.create_command_encoder(&Default::default());

        let mut renderpass: wgpu::RenderPass<'_> =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        // renderpass.draw(0..3, 0..1);
        renderpass.set_pipeline(&self.render_pipeline); // 2.
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        renderpass.draw(0..self.num_vertices, 0..1); // 3.

        // text_renderer.render(
        //     &mut atlas,
        //     &device,
        //     &queue,
        //     &mut pass,
        //     glyphon::Resolution {
        //         width: surface_config.width,
        //         height: surface_config.height,
        //     },
        //     &[(
        //         &buffer,
        //         glyphon::TextArea {
        //             left: 50.0,
        //             top: 50.0,
        //             scale: 1.0,
        //             bounds: glyphon::TextBounds {
        //                 left: 0,
        //                 top: 0,
        //                 right: surface_config.width as i32,
        //                 bottom: surface_config.height as i32,
        //             },
        //             default_color: glyphon::Color::rgb(255, 255, 255), // white
        //             ..Default()
        //         },
        //     )],
        // );

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                // Emits a new redraw requested event.
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            _ => (),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
   
    Vertex {
        position: [0.0, 0.5 + 0.3, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5 + 0.3, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5 + 0.3, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];
