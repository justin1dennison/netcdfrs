use std::collections::HashMap;
pub fn typemap() -> HashMap<u8, (&'static str, u8)> {
    [
        (1, ("b", 1)),
        (2, ("c", 1)),
        (3, ("h", 2)),
        (4, ("i", 4)),
        (5, ("f", 4)),
        (6, ("d", 8)),
    ].iter()
        .cloned()
        .collect()
}
