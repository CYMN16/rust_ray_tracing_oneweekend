use crate::camera::*;
use crate::hittable_list::*;
use crate::Image;
use druid::widget::*;
use druid::Data;
use druid::*;
use im::vector;
use im::Vector;

use std::sync::Arc;
use std::time::Instant;

#[derive(Data, Clone, Lens)]
struct AppState {
    image_buf: ImageBuf,
    cam: Camera,
    world: HittableList,
}

impl AppState {
    pub fn update_image(&mut self) {
        let img:Image = render(&mut self.cam, &self.world).into();
        self.image_buf = img.into();
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
        // data.update_image();
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

fn render(cam: &mut Camera, world: &HittableList) -> Image {
    let time_start = Instant::now();
    let img = cam.parallel_render(world);
    let time_end = Instant::now();

    println!("Time start: {:?}", time_start);
    println!("Time end: {:?}", time_end);
    println!("Time elapsed: {:?}", time_end - time_start);
    img
}
fn build_ui(app_state: &AppState, cam: &mut Camera, world: &HittableList) -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(Rebuilder::new().center(), 1.0
                )
                .with_child(
                    // Render Button
                    Button::new("Render").on_click(move |ctx, app_state: &mut AppState, _| {
                        // let img = render(&mut app_state.cam, &app_state.world).into();
                        app_state.update_image();
                        // ctx.request_update();
                    }),
                )
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .main_axis_alignment(MainAxisAlignment::Center),
        )
        .padding(10.0)
}

pub fn display_image(cam: &mut Camera, world: &HittableList) {
    let time_start = Instant::now();
    let img = render(cam, world);
    let time_end = Instant::now();

    let initial_data = AppState {
        image_buf: img.into(),
        cam: cam.clone(),
        world: world.clone(),
    };

    println!("Time start: {:?}", time_start);
    println!("Time end: {:?}", time_end);
    println!("Time elapsed: {:?}", time_end - time_start);
    // initial_data.image_buf = convert_im_to_druidim(img);

    let main_window = WindowDesc::new(build_ui(&initial_data, cam, world))
        .window_size((900.0, 600.0))
        .title("Ray tracing in one weekend");

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
