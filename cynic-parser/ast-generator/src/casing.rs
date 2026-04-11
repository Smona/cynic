pub fn to_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    // Setting this to true to avoid adding underscores at the beginning
    let mut prev_is_upper = true;
    for c in s.chars() {
        if c.is_uppercase() && !prev_is_upper {
            buf.push('_');
            buf.extend(c.to_lowercase());
            prev_is_upper = true;
        } else if c.is_uppercase() {
            buf.extend(c.to_lowercase());
        } else {
            prev_is_upper = false;
            buf.push(c);
        }
    }
    buf
}
