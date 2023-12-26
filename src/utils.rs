pub fn extract_domain_name(src: &str) -> &str {
    let mut found_pos = 0;
    let mut found_pos_prev = 0;

    let src_bytes = src.as_bytes();
    for i in 0..src_bytes.len() {
        if src_bytes[i] == b'.' {
            found_pos_prev = found_pos;
            found_pos = i;
        }
    }

    if src_bytes[found_pos_prev] == b'.' {
        return &src[found_pos_prev + 1..];
    }

    &src[found_pos_prev..]
}

#[cfg(test)]
mod tests {
    use crate::utils::extract_domain_name;

    #[test]
    fn test_find_domain_name() {
        assert_eq!(extract_domain_name("google.com"), "google.com");
        assert_eq!(extract_domain_name("test.google.com"), "google.com");
        assert_eq!(extract_domain_name("test.test.google.com"), "google.com");
    }
}
