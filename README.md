# Snor
Wrapper to standard library sleep functionality. 

## Why
I find myself always needing the sleep functionality given by the standard
library, but I always need to search how it works, so I made this to make my
life easier (_not_ that it contributes much).

## Usage
In `Cargo.toml`:
```
[dependencies]
snor = "1.0.0"
```

Used in a crate as:
```rust
use snor;

// Sleep for 10 seconds.
snor::sleep_sec(10);

// Sleep for 100 milliseconds.
snor::sleep_ms(100);

// Sleep for 1000 microseconds.
snor::sleep_micros(1000);

//Sleep for 10000 nanoseconds.
snor::sleep_nanos(10000);
```