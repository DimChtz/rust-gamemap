# rust-gamemap (gamemap) - v0.1.0
A small 2D Map library for console games in Rust.

# How to use
#### Create a map 20x20
```rust
let mut m = gamemap::Map::new(20, 20);
```
#### Update the map (print on the console)
```rust
 m.update();
```
#### Add some text above and below the map
```rust
let mut utext = String::from("This is some text above the map!!!");
let mut ltext = String::from("This is some text below the map!!!");

m.set_uppertext(utext)
 .set_lowertext(ltext)
 .update();
```
#### Add some sprites to the map (1 Player, 2 Enemies)
```rust
let mut sprites = vec![((15u32, 7u32), 'P'),
                       ((9u32, 12u32), 'E'),
                       ((11u32, 6u32), 'E')];

m.add_sprite(sprites[0])
 .add_sprite(sprites[1])
 .add_sprite(sprites[2])
 .update();
```
#### Move a sprite on the map
```rust
m.move_sprite(sprites[0].0, (1u32, 1u32))
 .update();
```
#### Remove sprites (Enemies)
```rust
m.remove_sprite(sprites[1].0)
 .remove_sprite(sprites[2].0)
 .update();
```
#### Clear the map
```rust
m.clear()
 .update();
```

# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
gamemap = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate gamemap;
```
