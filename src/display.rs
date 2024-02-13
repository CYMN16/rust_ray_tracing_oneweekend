use crate::camera::*;
use crate::hittable_list::*;
use crate::image::*;
use raylib::ffi::KeyboardKey;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

// fn draw_circles<'a>(frame: &Vec<Vec<i32>>, d: &mut RaylibDrawHandle) -> i32 {
//     for c in frame {
//         d.draw_circle(c[0], c[1], 10f32, raylib::prelude::Color{r:0,g:0,b:0,a:255});
//     }

//     0
// }

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;

pub fn display_image(cam: &mut crate::camera::Camera, world: &HittableList) {
    // let frame = vec![vec![100,200],vec![300,200],vec![100,300]];
    let img = cam.parallel_render(world);
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("RAY TRACING TESTING")
        .build();
    // let rl_img = raylib::ffi::Image{ data: img, width: img.width() as i32, height: img.height() as i32, mipmaps: 1, format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32 as i32};

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            break;
        }
        // if rl.is_key_pressed(KeyboardKey::KEY_J) {movement = movement/2f32}
        // if rl.is_key_pressed(KeyboardKey::KEY_K) {movement = movement*2f32}

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(raylib::prelude::Color::WHITE);

        // draw_circles(&frame, &mut d);
    }
}
