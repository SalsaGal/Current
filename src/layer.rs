//! Handling of game layers, the primary organisation system of Current.

use crate::GlobalData;
use crate::audio::AudioHandler;
use crate::graphics::GraphicsHandler;
use crate::input::InputHandler;

/// The trait that is actually used to create usable game layers.
pub trait GameLayer {
	/// Called when this layer is added to the stack.
	fn on_push(&mut self, _: &mut GlobalData) {}
	/// Called when this layer is taken off the stack.
	fn on_pop(&mut self, _: &mut GlobalData) {}
	/// Called when another layer is put on top of this.
	fn on_lose_focus(&mut self, _: &mut GlobalData) {}
	/// Called when another layer is taken off from on top of this.
	fn on_gain_focus(&mut self, _: &mut GlobalData) {}
	/// Called by the `Engine::update` function.
	fn update(&mut self, _: &mut GlobalData, _: &mut AudioHandler, _: &mut GraphicsHandler, _: &InputHandler, _: bool) -> Transition { Transition::None }
}

/// A simple layer with no programming made primarily for testing purposes.
pub struct EmptyLayer;
impl GameLayer for EmptyLayer {}

/// Transitions are used to determine the flow of the program and changes to the layer stack.
/// 
/// Transitions are returned by `GameLayer::update` and then applied as soon the function returns.
pub enum Transition {
	/// No change is to occure.
	None,
	/// The highest layer is to be removed from the layer stack.
	Pop,
	/// The highest `usize` layers are to be removed from the layer stack.
	PopMulti(usize),
	/// This layer is to be pushed to the layer stack.
	Push(Box<dyn GameLayer>),
	/// These layers are to be pushed to the layer stack.
	PushMulti(Vec<Box<dyn GameLayer>>),
	/// The program is to end. This also involves pushing all stacks, so their `on_pop` and `on_gain_focus` functions will be called.
	Quit,
	/// The entire layer stack gets popped, then replaced with this new stack. Internally all layers will be individually
	/// popped then pushed, so all the expected functions will get called.
	Replace(Vec<Box<dyn GameLayer>>),
}
