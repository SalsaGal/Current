use crate::GlobalData;
use crate::audio::AudioHandler;
use crate::graphics::GraphicsHandler;
use crate::input::InputHandler;

pub trait GameLayer {
	// When the layer is added to the stack
	fn on_push(&mut self, _data: &mut GlobalData) {}
	// When the layer is taken off the stack
	fn on_pop(&mut self, _data: &mut GlobalData) {}
	// When a layer is put on top of it
	fn on_lose_focus(&mut self, _data: &mut GlobalData) {}
	// WHen a layer is taken off from on top of it
	fn on_gain_focus(&mut self, _data: &mut GlobalData) {}
	
	fn update(&mut self, _data: &mut GlobalData, _audio: &mut AudioHandler, _graphics: &mut GraphicsHandler, _input: &InputHandler, _focused: bool) -> Transition { Transition::None }
}

pub struct EmptyLayer;
impl GameLayer for EmptyLayer {
	// When the layer is added to the stack
	fn on_push(&mut self, _data: &mut GlobalData) {}
	// When the layer is taken off the stack
	fn on_pop(&mut self, _data: &mut GlobalData) {}
	// When a layer is put on top of it
	fn on_lose_focus(&mut self, _data: &mut GlobalData) {}
	// WHen a layer is taken off from on top of it
	fn on_gain_focus(&mut self, _data: &mut GlobalData) {}
	
	fn update(&mut self, _data: &mut GlobalData, _audio: &mut AudioHandler, _graphics: &mut GraphicsHandler, _input: &InputHandler, _focused: bool) -> Transition { Transition::None }
}

pub enum Transition {
	None,
	Pop,
	PopMulti(usize),
	Push(Box<dyn GameLayer>),
	PushMulti(Vec<Box<dyn GameLayer>>),
	Quit,
	Replace(Vec<Box<dyn GameLayer>>),
}