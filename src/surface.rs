use {
    crate::{
        events::{Event, WindowEvents},
        prelude::*,
    },
    futures::future::{lazy, FutureObj},
    gleam::gl,
    glutin::{GlContext, GlWindow},
    webrender::api::*,
    webrender::ShaderPrecacheFlags,
    winit::WindowId,
};

// FIXME: fns that take children work with salsa
pub fn surface(compose: &impl Surface, key: ScopeId) {
    // get the state port for the whole scope
    let compose = compose.scope(key);
    surface_impl(compose);
}

fn handle_events(window_id: WindowId, mut events: WindowEvents) -> FutureObj<'static, ()> {
    Box::new(
        async move {
            while let Some(event) = await!(events.next()) {
                // TODO handle events?
            }
        },
    )
    .into()
}

pub fn surface_impl(compose: Scope) {
    let key = compose.id;
    let window_and_notifier = compose.state(callsite!(key), || {
        let mut events = WindowEvents::new();

        info!("initializing window");
        let window = GlWindow::new(
            winit::WindowBuilder::new()
                .with_title("moxie is alive?")
                .with_multitouch()
                .with_dimensions(winit::dpi::LogicalSize::new(1920.0, 1080.0)),
            glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            }),
            events.raw_loop(),
        )
        .unwrap();

        let window_id = window.id();
        info!("making window {:?} the current window", window_id);
        unsafe {
            window.make_current().ok();
        }

        // this notifier needs to be created before events is captured by the move block below
        let notifier = events.notifier();

        compose.task(callsite!(key), handle_events(window_id, events));

        (window, notifier)
    });
    let (window, notifier) = &*window_and_notifier;

    let gl = compose.state(callsite!(key), || match window.get_api() {
        glutin::Api::OpenGl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::OpenGlEs => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::WebGl => unimplemented!(),
    });

    let device_pixel_ratio = window.get_hidpi_factor() as f32;
    let framebuffer_size = {
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(device_pixel_ratio as f64);
        DeviceIntSize::new(size.width as i32, size.height as i32)
    };

    // TODO split returned state tuples?
    let mut renderer = compose.state(callsite!(key), || {
        debug!("creating webrender renderer");
        info!("OpenGL version {}", gl.get_string(gl::VERSION));
        info!("Device pixel ratio: {}", device_pixel_ratio);

        webrender::Renderer::new(
            gl.clone(),
            (*notifier).clone(),
            webrender::RendererOptions {
                precache_flags: ShaderPrecacheFlags::EMPTY,
                device_pixel_ratio,
                clear_color: Some(ColorF::new(0.0, 0.4, 0.3, 1.0)),
                ..webrender::RendererOptions::default()
            },
            None,
        )
        .unwrap()
    });

    let api = compose.state(callsite!(key), || renderer.1.create_api());

    let document_id = compose.state(callsite!(key), || api.add_document(framebuffer_size, 0));

    let epoch = Epoch(0);
    let pipeline_id = PipelineId(0, 0);
    let layout_size = framebuffer_size.to_f32() / euclid::TypedScale::new(device_pixel_ratio);

    let builder = DisplayListBuilder::new(pipeline_id, layout_size);

    trace!("new render xact, generating frame");
    let mut txn = Transaction::new();
    txn.set_root_pipeline(pipeline_id);
    txn.generate_frame();

    // FIXME render child functions here

    trace!("setting display list, generating frame, and sending transaction");
    txn.set_display_list(
        epoch,
        Some(ColorF::new(0.3, 0.0, 0.0, 1.0)),
        layout_size,
        builder.finalize(),
        true,
    );
    txn.generate_frame();
    api.send_transaction(*document_id, txn);
    renderer.0.update();

    trace!("rendering");
    renderer.0.render(framebuffer_size).unwrap();
    let _ = renderer.0.flush_pipeline_info();

    trace!("swapping buffers");
    window.swap_buffers().unwrap();
}
