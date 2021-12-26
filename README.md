## About
Based on [https://github.com/francoposa/programming-rust/blob/main/ch02-a-tour-of-rust/mandelbrot/src/main.rs](https://github.com/francoposa/programming-rust/blob/main/ch02-a-tour-of-rust/mandelbrot/src/main.rs) using `rayon` instead of `crossbeam` as illustrated in the book. Running the same example as the book, `rayon` version takes 0.4+ seconds and `crossbeam` one takes 0.8+ seconds.

## Example run
```bash
cargo build --release
 time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 
 ```
