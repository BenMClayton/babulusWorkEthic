use sdl2::event::Event;

use crate::winsdl::Winsdl;

mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new(200,200).unwrap();
}
