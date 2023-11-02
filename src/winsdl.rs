use sdl2::Sdl;

pub struct Winsdl{
    pub sdl: Sdl,
    pub win: Window,
    pub event_pump: EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl: sdl2::init().unwrap();
        let video_subsystem: = sdl.video().unwrap();

        let win: = video_subsystem
            .window("TEST", width as u32, height as u32)
            .build()    
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(Winsdl {
            sdl,
            win,
            event_pump,
        })
    };
}