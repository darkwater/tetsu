use std::ffi::{c_void, CStr, CString};

use libmpv2::{
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
    Mpv,
};

use self::app::MyApp;

mod app;

struct GlContext<'a> {
    get_proc_address: &'a dyn Fn(&CStr) -> *const c_void,
}

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        window_builder: Some(Box::new(|wb| wb.with_app_id("darkplayer"))),
        ..Default::default()
    };

    eframe::run_native(
        "Darkplayer",
        options,
        Box::new(|cc| {
            let mut mpv = Mpv::with_initializer(|init| {
                init.set_property("vo", "libmpv")?;
                init.set_property("ao", "pipewire")?;
                init.set_property("video-timing-offset", 0)?;
                Ok(())
            })
            .unwrap();

            mpv.event_context_mut().disable_deprecated_events().unwrap();

            let mut render_context = RenderContext::new(
                unsafe { mpv.ctx.as_mut() },
                vec![
                    RenderParam::ApiType(RenderParamApiType::OpenGl),
                    RenderParam::InitParams(OpenGLInitParams {
                        ctx: GlContext {
                            get_proc_address: cc.get_proc_address.unwrap(),
                        },
                        get_proc_address: |ctx, name| {
                            (ctx.get_proc_address)(&CString::new(name).unwrap()) as *mut _
                        },
                    }),
                ],
            )
            .expect("Failed creating render context");

            let ctx = cc.egui_ctx.clone();

            render_context.set_update_callback(move || {
                ctx.request_repaint();
            });

            Ok(Box::new(MyApp::new(mpv, render_context, cc)))
        }),
    )
}
