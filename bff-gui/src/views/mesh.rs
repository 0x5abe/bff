use eframe::egui;
use three_d::*;

pub struct MeshView {
    model: CpuModel,
}

impl MeshView {
    pub fn new(model: CpuModel) -> Self {
        Self { model }
    }
}

impl egui::Widget for MeshView {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::canvas(ui.style())
            .show(ui, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

                // let ang =
                //     match ui.memory(|mem| mem.data.get_temp::<Arc<Mutex<f32>>>(self.id_source)) {
                //         Some(val) => *val.lock().unwrap(),
                //         None => self.angle,
                //     };
                // self.angle = ang + response.drag_delta().x * 0.01;
                // ui.memory_mut(|mem| {
                //     mem.data
                //         .insert_temp(self.id_source, Arc::new(Mutex::new(self.angle)))
                // });

                let angle = response.drag_delta() * 0.05;

                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                        move |info, painter| {
                            with_three_d(&self.model, painter.gl(), |three_d| {
                                three_d.frame(
                                    ConvertedFrameInput::new(&three_d.context, &info, painter),
                                    angle,
                                );
                            });
                        },
                    )),
                };
                ui.painter().add(callback);
            })
            .response
    }
}

fn with_three_d<R>(
    cpu_model: &CpuModel,
    gl: &std::sync::Arc<eframe::glow::Context>,
    f: impl FnOnce(&mut ThreeDApp) -> R,
) -> R {
    use std::cell::RefCell;
    thread_local! {
        pub static THREE_D: RefCell<Option<ThreeDApp>> = RefCell::new(None);
        pub static MODEL: RefCell<Option<CpuModel>> = RefCell::new(None);
    }

    THREE_D.with(|three_d| {
        MODEL.with(|model| {
            let mut three_d = three_d.borrow_mut();
            let mut model = model.borrow_mut();
            if let Some(m) = model.as_ref() {
                if m.name != cpu_model.name {
                    model.replace(cpu_model.clone());
                    three_d.as_mut().unwrap().set_model(cpu_model, gl.clone())
                }
            } else {
                model.replace(cpu_model.clone());
            }
            let new = three_d.get_or_insert(ThreeDApp::new(cpu_model, gl.clone()));
            f(new)
        })
    })
}

///
/// Translates from egui input to three-d input
///
pub struct ConvertedFrameInput<'a> {
    screen: three_d::RenderTarget<'a>,
    viewport: three_d::Viewport,
    scissor_box: three_d::ScissorBox,
}

impl ConvertedFrameInput<'_> {
    pub fn new(
        context: &three_d::Context,
        info: &egui::PaintCallbackInfo,
        painter: &eframe::egui_glow::Painter,
    ) -> Self {
        use three_d::*;

        // Disable sRGB textures for three-d
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unsafe_code)]
        unsafe {
            use eframe::glow::HasContext as _;
            context.disable(eframe::glow::FRAMEBUFFER_SRGB);
        }

        // Constructs a screen render target to render the final image to
        let screen = painter.intermediate_fbo().map_or_else(
            || {
                RenderTarget::screen(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                )
            },
            |fbo| {
                RenderTarget::from_framebuffer(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                    fbo,
                )
            },
        );

        // Set where to paint
        let viewport = info.viewport_in_pixels();
        let viewport = Viewport {
            x: viewport.left_px.round() as _,
            y: viewport.from_bottom_px.round() as _,
            width: viewport.width_px.round() as _,
            height: viewport.height_px.round() as _,
        };

        // Respect the egui clip region (e.g. if we are inside an `egui::ScrollArea`).
        let clip_rect = info.clip_rect_in_pixels();
        let scissor_box = ScissorBox {
            x: clip_rect.left_px.round() as _,
            y: clip_rect.from_bottom_px.round() as _,
            width: clip_rect.width_px.round() as _,
            height: clip_rect.height_px.round() as _,
        };
        Self {
            screen,
            scissor_box,
            viewport,
        }
    }
}

pub struct ThreeDApp {
    context: Context,
    camera: Camera,
    pub model: Model<NormalMaterial>,
}

impl ThreeDApp {
    pub fn new(cpu_model: &CpuModel, gl: std::sync::Arc<eframe::glow::Context>) -> Self {
        let context = Context::from_gl_context(gl).unwrap();
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            vec3(0.0, 0.0, 5.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(70.0),
            0.1,
            10.0,
        );
        let model = Model::new(&context, cpu_model).unwrap();
        Self {
            context,
            camera,
            model,
        }
    }

    pub fn set_model(&mut self, cpu_model: &CpuModel, gl: std::sync::Arc<eframe::glow::Context>) {
        let context = Context::from_gl_context(gl).unwrap();
        self.model = Model::new(&context, cpu_model).unwrap();
    }

    pub fn frame(
        &mut self,
        frame_input: ConvertedFrameInput<'_>,
        angle: egui::Vec2,
    ) -> Option<eframe::glow::Framebuffer> {
        self.camera.set_viewport(frame_input.viewport);

        self.camera
            .rotate_around_with_fixed_up(&Vector3::zero(), angle.x, angle.y);

        frame_input
            .screen
            .clear_partially(frame_input.scissor_box, ClearState::depth(1.0))
            .render_partially(
                frame_input.scissor_box,
                &self.camera,
                &self.model,
                &[&AmbientLight::new(
                    &self.context,
                    1.0,
                    Srgba {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                )],
            );

        frame_input.screen.into_framebuffer()
    }
}
