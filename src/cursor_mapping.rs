use std::collections::HashMap;

/// Mapping from Windows cursor names to X11 cursor names
pub fn get_windows_to_x11_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    map.insert("Normal", "left_ptr");
    map.insert("Link", "link");
    map.insert("Person", "pointer");
    map.insert("Handwriting", "pencil");
    map.insert("Text", "text");
    map.insert("Unavailable", "not-allowed");
    map.insert("Busy", "wait");
    map.insert("Working", "progress");
    map.insert("Precision", "crosshair");
    map.insert("Move", "move");
    map.insert("Alternate", "question_arrow");
    map.insert("Help", "help");
    map.insert("Pin", "pin");
    map.insert("Horizontal", "size_hor");
    map.insert("Vertical", "size_ver");
    map.insert("Diagonal1", "size_bdiag");
    map.insert("Diagonal2", "size_fdiag");
    
    map
}

/// Common cursor symlinks for compatibility
pub fn get_cursor_symlinks() -> Vec<(&'static str, &'static str)> {
    vec![
        // Basic cursor symlinks
        ("left_ptr", "arrow"),
        ("left_ptr", "default"),
        ("left_ptr", "top_left_arrow"),
        ("pointer", "hand1"),
        ("pointer", "hand2"),
        ("pointer", "pointing_hand"),
        ("pointer", "openhand"),
        ("pointer", "grab"),
        ("move", "fleur"),
        ("move", "all-scroll"),
        ("move", "size_all"),
        ("wait", "watch"),
        ("progress", "left_ptr_watch"),
        ("crosshair", "cross"),
        ("text", "xterm"),
        ("text", "ibeam"),
        ("pencil", "draft"),
        ("question_arrow", "help"),
        ("question_arrow", "whats_this"),
        ("question_arrow", "left_ptr_help"),
        ("not-allowed", "crossed_circle"),
        ("not-allowed", "forbidden"),
        ("not-allowed", "no_drop"),
        ("not-allowed", "dnd_no_drop"),
        ("size_hor", "sb_h_double_arrow"),
        ("size_hor", "h_double_arrow"),
        ("size_hor", "ew-resize"),
        ("size_hor", "col-resize"),
        ("size_hor", "split_h"),
        ("size_ver", "sb_v_double_arrow"),
        ("size_ver", "v_double_arrow"),
        ("size_ver", "ns-resize"),
        ("size_ver", "row-resize"),
        ("size_ver", "split_v"),
        ("size_bdiag", "fd_double_arrow"),
        ("size_bdiag", "nesw-resize"),
        ("size_fdiag", "bd_double_arrow"),
        ("size_fdiag", "nwse-resize"),
        ("left_ptr", "wayland-cursor"),
        
        // Additional common cursor IDs (hex-encoded)
        ("left_ptr_watch", "00000000000000020006000e7e9ffc3f"),
        ("left_ptr_watch", "08e8e1c95fe2fc01f976f1e063a24ccd"),
        ("left_ptr_watch", "3ecb610c1bf2410f44200f48c40d3599"),
        ("sb_v_double_arrow", "00008160000006810000408080010102"),
        ("sb_v_double_arrow", "2870a09082c103050810ffdffffe0204"),
        ("sb_h_double_arrow", "028006030e0e7ebffc7f7070c0600140"),
        ("sb_h_double_arrow", "14fef782d02440884392942c1120523"),
        ("crossed_circle", "03b6e0fcb3499374a867c041f52298f0"),
        ("question_arrow", "5c6cd98b3f3ebcb1f9c7f1c204630408"),
        ("question_arrow", "d9ce0ab605698f320427677b458ad60b"),
        ("move", "4498f0e0c1937ffe01fd06f973665830"),
        ("move", "9081237383d90e509aa00f00170e968f"),
        ("link", "3085a0e285430894940527032f8b26df"),
        ("link", "640fb0e74195791501fd1ed57b41487f"),
        ("link", "a2a266d0498c3104214a47bd64ab0fc8"),
        ("hand2", "9d800788f1b08800ae810202380a0822"),
        ("hand2", "e29285e634086352946a0e7090d73106"),
        ("size_fdiag", "c7088f0f3e6c8088236ef8e1e3e70000"),
        ("size_bdiag", "fcf1c3c7cd4491d801f1e1c78f100000"),
        ("copy", "1081e37283d90000800003c07f3ef6bf"),
        ("copy", "6407b0e94181790501fd1e167b474872"),
        ("copy", "b66166c04f8c3109214a4fbd64a50fc8"),
        ("dnd-link", "alias"),
        ("plus", "cell"),
        ("grabbing", "closedhand"),
        ("tcross", "color-picker"),
        ("cross", "cross_reverse"),
        ("cross", "diamond_cross"),
        ("grabbing", "dnd-move"),
        ("grabbing", "dnd-none"),
        ("dotbox", "dot_box_mask"),
        ("sb_v_double_arrow", "double_arrow"),
        ("sb_down_arrow", "down-arrow"),
        ("right_ptr", "draft_large"),
        ("right_ptr", "draft_small"),
        ("dotbox", "draped_box"),
        ("right_side", "e-resize"),
        ("grabbing", "fcf21c00b30f7e3f83fe0dfd12e71cff"),
        ("dotbox", "icon"),
        ("sb_left_arrow", "left-arrow"),
        ("top_right_corner", "ne-resize"),
        ("dnd_no_drop", "no-drop"),
        ("top_side", "n-resize"),
        ("top_left_corner", "nw-resize"),
        ("X_cursor", "pirate"),
        ("sb_right_arrow", "right-arrow"),
        ("bottom_right_corner", "se-resize"),
        ("sb_h_double_arrow", "size-hor"),
        ("sb_v_double_arrow", "size-ver"),
        ("bottom_side", "s-resize"),
        ("bottom_left_corner", "sw-resize"),
        ("dotbox", "target"),
        ("sb_up_arrow", "up-arrow"),
        ("left_side", "w-resize"),
        ("X_cursor", "x-cursor"),
    ]
}

/// Get hotspot ratios for different cursor types
pub fn get_cursor_hotspot(cursor_name: &str) -> (f64, f64) {
    match cursor_name {
        "left_ptr" | "not-allowed" | "unavailable" => (0.125, 0.125), // Top-left tip
        "text" | "xterm" | "ibeam" => (0.5, 0.5), // Center
        name if name.starts_with("pointer") || name.starts_with("hand") => (0.3, 0.125), // Fingertip
        "pencil" => (0.125, 0.125), // Tip
        "move" => (0.5, 0.5), // Center
        name if name.starts_with("size_") => (0.5, 0.5), // Center for resize cursors
        _ => (0.5, 0.5), // Default: center
    }
}
