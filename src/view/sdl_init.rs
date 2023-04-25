use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn sdl_init(titre: &str, height:&u32, width: &u32)->(Sdl, VideoSubsystem, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_sudsytheme = sdl_context.video().unwrap();

    let window = video_sudsytheme.window(&titre, *width, *height)
                    .position_centered()
                    .build()
                    .unwrap();

    let canvas = window
                .into_canvas()
                .build()
                .unwrap();
    (sdl_context, video_sudsytheme, canvas)
}

pub fn ttf_int () -> Sdl2TtfContext{
    sdl2::ttf::init().map_err(|e| e.to_string()).unwrap()
}