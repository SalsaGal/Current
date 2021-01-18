use crate::graphics::GraphicsHandler;
use crate::input::InputHandler;

pub trait GameLayer {
	// When the layer is added to the stack
	fn on_push(&mut self) {}
	// When the layer is taken off the stack
	fn on_pop(&mut self) {}
	
	fn update(&mut self, _graphics: &mut GraphicsHandler, _input: &InputHandler, _focused: bool) -> Transition { Transition::None }
}

pub enum Transition {
	None,
	Pop,
	Push(Box<dyn GameLayer>),
	Quit,
}