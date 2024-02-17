use crate::camera::*;
use crate::hittable_list::*;
use crate::utility::*;
use crate::Image;
use crate::Vec3;
use druid::widget::*;
use druid::Data;
use druid::*;
use im::vector;
use im::Vector;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Data, Clone, Lens)]
struct AppState {
    image_buf: ImageBuf,
    cam: Camera,
    world: HittableList,
    time_elapsed: Duration,
    continous_render: bool,
}

unsafe impl Send for AppState {}

impl AppState {
    pub fn update_image(&mut self) {
        let distance = 5.;
        let da = 0.2;
        let cam = &self.cam;
        let lookfrom = &cam.lookfrom;
        let lookat = &cam.lookat;

        let dx = lookfrom.x - lookat.x;
        let dz = lookfrom.z - lookat.z;

        let mut angle = dz.atan2(dx);

        angle = angle + da;

        // println!("angle: {}",angle);

        self.cam.lookfrom = Vec3::new(
            lookat.x + distance * angle.cos(),
            lookfrom.y,
            lookat.z + distance * angle.sin(),
        );

        // let prev_loc = &self.cam.lookfrom;
        // self.cam.lookfrom = prev_loc + 0.01 * prev_loc;

        let time_start = Instant::now();
        let img = self.cam.parallel_render(&self.world);
        let time_end = Instant::now();

        // println!("Time start: {:?}", time_start);
        // println!("Time end: {:?}", time_end);
        let time_elapsed = time_end - time_start;
        println!("Time elapsed: {:?}", time_elapsed);
        self.image_buf = img.into();
        self.time_elapsed = time_elapsed;
    }
}

/// builds a child Flex widget from some parameters.
struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(data);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }

        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget(state: &AppState) -> Box<dyn Widget<AppState>> {
    let png_data = state.image_buf.clone();

    let img = druid::widget::Image::new(png_data);

    let sized = SizedBox::new(img);

    sized.border(Color::grey(0.6), 2.0).center().boxed()
}

fn build_ui(app_state: &AppState, cam: &mut Camera, world: &HittableList) -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    Flex::column()
                        .with_child(Rebuilder::new().center())
                        .with_child(Label::dynamic(|app_state: &AppState, _| {
                            format!("Frame time: {:?}", app_state.time_elapsed)
                        })), FlexParams::new(1.0, CrossAxisAlignment::Fill)
                )
                .with_child(
                    Flex::column()
                        .with_child(
                            // Render Button
                            Button::new("Render").on_click(
                                move |ctx, app_state: &mut AppState, _| {
                                    app_state.update_image();
                                },
                            ),
                        )
                        .with_spacer(10.)
                        .with_child(Label::new("Continous rendering"))
                        .with_child(LensWrap::new(Switch::new(), AppState::continous_render)),
                )
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .main_axis_alignment(MainAxisAlignment::Center),
        )
        .padding(10.0)
}

fn continous_rendering(event_sink: druid::ExtEventSink) {
    thread::sleep(Duration::from_millis(250));

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.update_image();
    });
    loop {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            if data.continous_render {
                data.update_image();
            }
        });
        thread::sleep(Duration::from_millis(250));
    }
}

pub fn display_image(cam: &mut Camera, world: &HittableList) {
    // let time_start = Instant::now();
    // let img = render(cam, world);
    // let time_end = Instant::now();
    let img = ImageBuf::empty();
    let initial_data = AppState {
        image_buf: img.into(),
        cam: cam.clone(),
        world: world.clone(),
        time_elapsed: Duration::new(0, 0),
        continous_render: false,
    };

    // println!("Time start: {:?}", time_start);
    // println!("Time end: {:?}", time_end);
    // println!("Time elapsed: {:?}", time_end - time_start);

    let main_window = WindowDesc::new(build_ui(&initial_data, cam, world))
        .window_size((900.0, 600.0))
        .title("Ray tracing in one weekend");

    let launcher = AppLauncher::with_window(main_window);

    let eventsink = launcher.get_external_handle();

    thread::spawn(move || {
        continous_rendering(eventsink);
    });

    launcher
        .launch(initial_data)
        .expect("Failed to launch application");
}
