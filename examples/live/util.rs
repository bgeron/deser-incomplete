pub const MAXIMUM_SIZE_OF_CODEPOINT: usize = 4;

/// Try to parse initial segments from the bytes, until something parses, and then
/// remove that segment from the buffer.
pub fn pop_parsed_from_front<T>(
    buf: &mut Vec<u8>,
    max_length: usize,
    mut parse: impl FnMut(&[u8]) -> Option<T>,
) -> Option<T> {
    // Characters in UTF-8 are at most 4 bytes.
    for end_index in 1..=max_length.min(buf.len()) {
        if let Some(value) = parse(&buf[..end_index]) {
            // Remove this character from the front. Linear time in size
            // of the buffer, so use a small buffer.
            buf.drain(..end_index);
            return Some(value);
        }
    }
    None
}
