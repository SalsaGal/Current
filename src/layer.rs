use crate::GlobalData;
use crate::graphics::GraphicsHandler;
use crate::input::InputHandler;

pub trait GameLayer {
	// When the layer is added to the stack
	fn on_push(&mut self) {}
	// When the layer is taken off the stack
	fn on_pop(&mut self) {}
	// When a layer is put on top of it
	fn on_lose_focus(&mut self) {}
	// WHen a layer is taken off from on top of it
	fn on_gain_focus(&mut self) {}
	
	fn update(&mut self, _data: &mut GlobalData, _graphics: &mut GraphicsHandler, _input: &InputHandler, _focused: bool) -> Transition { Transition::None }
}

pub enum Transition {
	None,
	Pop,
	Push(Box<dyn GameLayer>),
	Quit,
}