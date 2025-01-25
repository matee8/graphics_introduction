# Figura 🎨

**2D geometric primitives with backend-agnostic rendering.**

<img src="https://github.com/matee8/figura/raw/main/assets/heart-example.png" width="400" alt="Heart curve example">

## Features

- **SDL2 Integration**: Built-in support for hardware-accelerated rendering.
- **Parametric Curves**: Create complex shapes using mathematical functions.
- **Geometric Primitives**:
  - Circles, polygons, Hermite arcs
  - Line segments with clipping/intersection detection
- **Extensible Architecture**: Custom renderer backend support.
- **Precision Math**: Floating-point accuracy with error margins.

## Usage

### Basic Curve Rendering
```rust
use figura::{Color, OneColorCurve, Renderable};

// Create a circle parametric curve
let circle = OneColorCurve::new_parametric(
    Color::RED,
    |t| 200.0 * t.cos() + 320.0,
    |t| 200.0 * t.sin() + 240.0,
    0.0,
    2.0 * std::f64::consts::PI,
    None,
)?;

// Render to SDL2 canvas
circle.render(&mut canvas)?;
```

### Polygon Operations
```rust
use figura::{Polygon, Point};

// Create a quadrilateral
let polygon = Polygon::new(
    &[
        Point::new(100.0, 100.0),
        Point::new(100.0, 200.0),
        Point::new(200.0, 200.0),
        Point::new(200.0, 100.0),
    ],
    Color::BLUE,
)?;

// Check point containment
assert!(polygon.contains(Point::new(150.0, 150.0)));
```

## Examples

| Example       | Description                          | Command                                      |
|---------------|--------------------------------------|----------------------------------------------|
| Circle        | Basic parametric circle rendering    | `cargo run --features sdl2 --example circle` |
| Epicycloid    | Complex parametric curve             | `cargo run --features sdl2 --example epicycloid -- -a 5 -b 3` |
| Heart         | Romantic curve demonstration         | `cargo run --features sdl2 --example heart`  |

## API Overview

### Core Components
- **`OneColorCurve`**: Parametric curve primitive
- **`Polygon`**: Closed shape with containment checks
- **`OneColorSegment`**: Line segment with clipping support
- **`HermiteArc`**: Smooth curve interpolation between points

### Key Traits
- `GeometricPrimitive`: Base trait for all shapes
- `Renderable`: Unified rendering interface
- `Shape`: Polygon operations and properties

## Contributing

Contributions welcome! Please follow:
1. Rust API Guidelines
2. Format code with `rustfmt` before commmiting
3. Use `clippy` to analyse code before commiting
4. Conventional Commits specification

```bash
cargo test --all-features
cargo clippy
cargo +nightly fmt
```

## License

MIT License © 2025 matee8
