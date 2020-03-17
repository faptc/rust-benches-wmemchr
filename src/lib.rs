// #[cfg(not(windows))]
// compile_error!("This crate only works on Windows.");

// Ported from <https://github.com/bminor/glibc/blob/master/wcsmbs/wmemchr.c>.
pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut len = haystack.len();
    let mut start = &haystack[..];

    // For performance reasons unfold the loop eight times.
    let mut chunks_8 = haystack.chunks_exact(8);
    for chunk in chunks_8 {
        let mut iter = chunk.iter().enumerate();
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some((i, c)) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
    }
    None
}
