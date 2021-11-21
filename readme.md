## Collection of methods to calculate golden ratio number

### How to use this library
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    goldenratio = {git = "https://github.com/azavgo/goldenratio"}
```
2. Use one of the methods from the library collection to calculate golden ratio number: 
```Rust
use goldenratio::GoldenRatio;  

fn main() {
    let gr = GoldenRatio::gr_sin(); 
    let f = gr.fi(); 
    println!("Golden ratio φ = {:?}", f);
}
```
