use crate::input::InputHandler;

pub trait GameLayer {
	// When the layer is added to the stack
	fn on_push(&mut self) {}
	// When the layer is taken off the stack
	fn on_pop(&mut self) {}
	
	// When the layer is at the top of the stack
	fn update_focused(&mut self, _input: &InputHandler) {}
	// When the layer is not at the top of the stack
	fn update_unfocused(&mut self, _input: &InputHandler) {}
	// When the layer is anywhere
	fn update(&mut self, _input: &InputHandler) {}
}