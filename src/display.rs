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
    pub fn update_image(&mut self, img: Image) {
        self.image_buf = img.into();
    }
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
                .with_child(
                    // Viewport
                    Container::new(druid::widget::Image::new(app_state.image_buf.clone()).center())
                    // druid::widget::Image::new(app_state.image_buf.clone())
                    .fix_size(800., 800.*9./16.)
                    .padding(10.0)
                    .border(Color::BLACK, 1.0)
                    .center()
                    .lens(AppState::image_buf)
                )
                .with_child(
                    // Render Button
                    Button::new("Render").on_click(move |ctx, app_state: &mut AppState, _| {
                        let img = render(&mut app_state.cam, &app_state.world).into();
                        app_state.update_image(img);
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
