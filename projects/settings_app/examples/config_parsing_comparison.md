# Config Parsing Comparison: C vs Rust

## Parsing a Hyprland config line in C:
```c
// Parse something like: "gaps_in = 5"
void parse_config_line(const char* line) {
    char key[256];
    char value[256];

    // Manual parsing, hope we don't overflow
    if (sscanf(line, "%255s = %255s", key, value) == 2) {
        if (strcmp(key, "gaps_in") == 0) {
            int gaps = atoi(value);  // No error checking
            // Apply setting...
        } else if (strcmp(key, "border_size") == 0) {
            int border = atoi(value);
            // Apply setting...
        }
        // ... many more strcmp chains
    }
    // What if format is different? What if there are spaces?
    // Need to manually handle all cases
}
```

## Parsing the same in Rust:
```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HyprlandConfig {
    gaps_in: Option<i32>,
    border_size: Option<i32>,
    // ... all fields typed and optional
}

fn parse_config(content: &str) -> Result<HyprlandConfig, Error> {
    // Automatic parsing with error handling
    toml::from_str(content)?  // Or custom parser
}
```

## Reading file in C:
```c
char* read_config_file(const char* path) {
    FILE* file = fopen(path, "r");
    if (!file) return NULL;

    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);

    char* buffer = malloc(size + 1);
    if (!buffer) {
        fclose(file);
        return NULL;
    }

    fread(buffer, 1, size, file);
    buffer[size] = '\0';
    fclose(file);

    return buffer;  // Caller must free()
}
```

## Reading file in Rust:
```rust
fn read_config_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)  // That's it, memory handled automatically
}
```

## The Real Difference:
It's not about speed - both will be equally fast. It's about:
- **Development speed**: Rust will be faster to develop safely
- **Maintenance**: Rust catches more bugs at compile time
- **Your comfort**: If you're more comfortable reading/modifying C, that matters!