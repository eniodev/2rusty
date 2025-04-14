use std::fs;

pub fn read_file(path: &str) -> String {
    let content = fs::read_to_string(path);
    let content = match content {
        Ok(content) => content,
        Err(e) => panic!("[Error] Invalid path: {} for {}", e, path),
    };

    content
}

#[macro_export]
macro_rules! either {
    ($condition:expr, $truely: expr, $falsy: expr) => {
        if $condition { $truely } else { $falsy }
    };
}
