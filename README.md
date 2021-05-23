# Current

A Rust game library built primarily off SDL2 with ease of use as a focus.

## Basic Window

With Current you can get running almost instantly:

```rust
use current::Engine;
use current::layer::EmptyLayer;
use current::math::Vector2;

fn main() {
  let mut engine = Engine::new("Window", Vector2::new(640, 480), Box::new(EmptyLayer));
  
  while engine.running {
    engine.update();
  }
}
```

## Layers

Current organises programs into layers. Internally, there is a stack of layers which all get updated from the bottom up. To use the layers you must first make a struct for it, then implement `GameLayer` on it. An example of a layer that prints text, then adds another layer when a key is pressed would be:

```rust
struct Display;
impl GameLayer for Display {
  fn update(&mut self, _: &mut GlobalData, _: &mut AudioHandler, _: &mut GraphicsHandler, input: &InputHandler, _: bool) -> Transition {
    println!("Updated")
    if input.key_is(Scancode::Escape, InputState::Pressed) {
      return Transition::Push(PauseScreen);
    }
    Transition::None
  }
}

struct PauseScreen;
impl GameLayer for PauseScreen {
  fn update(&mut self, _: &mut GlobalData, _: &mut AudioHandler, _: &mut GraphicsHandler, input: &InputHandler, _: bool) -> Transition {
    println!("Paused");
    if input.key_is(Scancode::Escape, InputState::Pressed) {
      return Transition::Pop;
    }
    Transition::None
  }
}
```



