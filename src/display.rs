use crate::camera::*;
use crate::hittable_list::*;
use druid::widget::*;
use druid::*;
use druid::Data;
use im::vector;
use im::Vector;
use crate::Image;

use std::sync::Arc;
use std::time::Instant;

#[derive(Data, Clone)]
struct AppState {
    image_buf: ImageBuf,
}

fn render(cam: &mut Camera, world: &HittableList) -> Image {
    cam.parallel_render(world)
}

fn convert_im_to_druidim(img: Image) -> ImageBuf{
    let width = img.width();
    let height = img.height();
    ImageBuf::from_raw(img, piet::ImageFormat::Rgb, width, height)    
}   
    

fn build_ui(app_state: AppState, cam: &mut Camera, world: &HittableList) -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    // Viewport
                    Container::new(
                        druid::widget::Image::new(app_state.image_buf)
                        .center(),
                    )
                        .fix_size(600., 700.)
                        .padding(10.0)
                        .border(Color::BLACK, 1.0)
                        .center(),
                )
                .with_child(
                    // Render Button
                    Button::new("Render").on_click(|_, _, _| {
                        
                        // app_state.image_buf = ImageBuf::from_raw(img, format, width, height);
                    }),
                )
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .main_axis_alignment(MainAxisAlignment::Center),
        )
        .padding(10.0)
}

pub fn display_image(cam: &mut Camera, world: &HittableList) {

    let img = render(cam, world);
    
    let time_start = Instant::now();

    let initial_data = AppState {
        image_buf: convert_im_to_druidim(img),
    };

    let time_end = Instant::now();

    println!("Time start: {:?}", time_start);
    println!("Time end: {:?}", time_end);
    println!("Time elapsed: {:?}", time_end - time_start);
    // initial_data.image_buf = convert_im_to_druidim(img);

    let main_window = WindowDesc::new(build_ui(initial_data.clone(), cam, world))
        .window_size((900.0, 800.0))
        .title("Ray tracing in one weekend");


    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
