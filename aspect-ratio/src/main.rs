use macroquad::prelude::*;


#[macroquad::main("aspect-ratio")]
async fn main() {
    let mut d = Dimensions::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        d.process_dimensions();
        
        next_frame().await
    }
}


struct Dimensions {
    w: f32,
    h: f32,
    fps: i32,
    last_w: f32,
    last_h: f32,
    last_fps: i32,
    text_params: TextParams,
}

impl Dimensions {
    fn new() -> Dimensions {
        let mut text_params = TextParams::default();
        text_params.color = Color::new(0.8, 0.9, 0.9, 1.0);
        Dimensions{
            w: 0.0,
            h: 0.0,
            fps: 0,
            last_w: 0.0,
            last_h: 0.0,
            last_fps: 0,
            text_params: text_params,
        }
    }

    fn process_dimensions(self: &mut Dimensions) {
        self.w = screen_width();
        self.h = screen_height();
        self.fps = get_fps();
        self.text_params.font_size = ((self.w + self.h) * 0.015) as u16;

        self.draw_dimensions();
        self.last_w = self.w;
        self.last_h = self.h;
        self.last_fps = self.fps;
    }

    fn draw_dimensions(self: &Dimensions) {
        let border = (self.w + self.h) * 0.01;
        draw_rectangle( border, border, self.w-border*2.0, border, DARKPURPLE);
        draw_rectangle( border, self.h-border*2.0, self.w-border*2.0, border, DARKPURPLE);
        draw_rectangle( border, border, border, self.h-border*2.0, DARKPURPLE);
        draw_rectangle( self.w-border*2.0, border, border, self.h-border*2.0, DARKPURPLE);

        let mut aspect_ratio = "Square";
        if self.w > self.h*1.1 {
            aspect_ratio = "Wide";
        } else if self.h > self.w*1.1 {
            aspect_ratio = "Tall";
        }

        let aspect_ratio_text = format!("Aspect Ratio {}", aspect_ratio);
        let dimensions_text = format!("Dimensions {}x{}px", self.w, self.h);
        let fps_text = format!("FPS {}", get_fps());

        let padding = border * 3.0;

        draw_text_ex(aspect_ratio_text.as_str(), padding, padding, self.text_params);
        draw_text_ex(dimensions_text.as_str(), padding, self.h - padding, self.text_params);
        let text_size = measure_text(fps_text.as_str(), Some(self.text_params.font), self.text_params.font_size, 1.0);
        draw_text_ex(fps_text.as_str(), (self.w - padding)  - text_size.width, self.h - padding, self.text_params);


    }
}