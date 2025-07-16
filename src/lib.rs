use zed_extension_api as zed;

struct Stan;

impl zed::Extension for Stan {
    fn new() -> Self {
        println!("[STAN EXTENSION] Hello, world!");
        Stan
    }
}

zed::register_extension!(Stan);
