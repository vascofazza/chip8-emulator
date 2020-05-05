pub mod display_interface;
pub mod input_interface;
pub mod audio_interface;

use display_interface::DisplayInterface;
use audio_interface::AudioInterface;
use input_interface::InputInterface;

use sdl2::EventPump;
use sdl2::event::Event;

pub struct InterfaceManager
{
    pub(crate) event_pump: EventPump,
    pub video_interface: DisplayInterface,
    pub audio_interface: AudioInterface,
    pub input_interface: InputInterface
}

impl InterfaceManager
{
    pub fn new() -> InterfaceManager
    {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let video_interface = DisplayInterface::new(&sdl_context);
        let audio_interface = AudioInterface::new(&sdl_context);
        let input_interface = InputInterface::new();
        InterfaceManager
        {
            event_pump,
            video_interface,
            audio_interface,
            input_interface
        }
    }

    pub fn run(&mut self) -> Option<Event>
    {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                   return Some(event);
                },
                _ => {}
            }
        }
        return None
    }
}