# clia-local-offset

Get current local timezone offset simplely.

## Usage

```rust
use tracing_subscriber::fmt::time::OffsetTime;

// Local offset timezone init, and set time format.
let offset = clia_local_offset::current_local_offset()
    .expect("Can not get local offset!");

let timer = OffsetTime::new(
    offset,
    format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
    ),
);
```
