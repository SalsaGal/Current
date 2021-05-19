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



