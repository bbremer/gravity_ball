use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::mouse::MouseButton;
use std::time::Duration;
use ndarray;
use ndarray_linalg;

fn draw<T: RenderTarget>(canvas: &mut Canvas<T>, &r: &Rect) {
    match canvas.fill_rect(r) {
        Ok(_v) => {},
        Err(v) => println!("bad: {}", v),
    };
}


struct Ball {
    r: Rect,
    v: ndarray::Array1<f32>,
}

 
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
       .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let black = Color::RGB(0, 0, 0);
    
    // walls
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let wall_n = Rect::new(0, 0, 800, 10);
    let wall_s = Rect::new(0, 590, 800, 10);
    let wall_e = Rect::new(790, 0, 10, 600);
    let wall_w = Rect::new(0, 0, 10, 600);

    let t_diff = 1_000_000_000u32 / 60;
    let t_diff2 = t_diff as f32;

    canvas.present();

    let max_speed = 0.000001;
    let change_speed = 0.000000005;

    let mut ball = Ball{r: Rect::new(10, 10, 50, 50), v: ndarray_linalg::array![0.0, 0.0]};

    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        let e = event_pump.mouse_state();
        if e.is_mouse_button_pressed(MouseButton::Left) {
            // ball.r.x = e.x() - (ball.r.width() as i32 / 2);
            // ball.r.y = e.y() - (ball.r.height() as i32 / 2);
            let ball_center_x = ball.r.x + (ball.r.width() as i32 / 2);
            let ball_center_y = ball.r.y + (ball.r.height() as i32 / 2);
            if ball_center_x > e.x() {
                ball.v[0] = ball.v[0] - change_speed;
            } else if ball_center_x < e.x() {
                ball.v[0] = ball.v[0] + change_speed;
            }
            if ball_center_y > e.y() {
                ball.v[1] = ball.v[1] - change_speed;
            } else if ball_center_y < e.y() {
                ball.v[1] = ball.v[1] + change_speed;
            }
        }

        if ball.v.norm() > max_speed {
            ball.v = ball.v / ball.v.norm * max_speed;
        }
        // if ball.v_x.abs() > max_speed {
        //     ball.v_x = ball.v_x.signum() * max_speed;
        // }
        // if ball.v_y.abs() > max_speed {
        //     ball.v_y = ball.v_y.signum() * max_speed;
        // }
        ball.r.x = ball.r.x + (t_diff2 * ball.v[0]) as i32;
        ball.r.y = ball.r.y + (t_diff2 * ball.v[1]) as i32;

        // wall collision detection
        if ball.r.has_intersection(wall_n) {
            ball.r.y = 10;
            ball.v[1] = - ball.v[1];
        }
        if ball.r.has_intersection(wall_s) {
            ball.r.y = 590 - ball.r.height() as i32;
            ball.v[1] = - ball.v[1];
        }
        if ball.r.has_intersection(wall_e) {
            ball.r.x = 790 - ball.r.width() as i32;
            ball.v[0] = - ball.v[0];
        }
        if ball.r.has_intersection(wall_w) {
            ball.r.x = 10;
            ball.v[0] = - ball.v[0];
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        draw(&mut canvas, &wall_n);
        draw(&mut canvas, &wall_s);
        draw(&mut canvas, &wall_e);
        draw(&mut canvas, &wall_w);
        draw(&mut canvas, &(ball.r));
        // match canvas.fill_rect(ball) {
        //     Ok(_v) => {},
        //     Err(v) => println!("bad: {}", v),
        // };
        
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ::std::thread::sleep(Duration::new(0, t_diff));
    }
}
