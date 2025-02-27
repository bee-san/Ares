use crate::storage::INVISIBLE_CHARS;

/// Test that checks if the invisible character detection works correctly
#[test]
fn test_invisible_character_detection() {
    // Get a zero width space character
    let zero_width_space = char::from_u32(0x200B).unwrap();
    assert!(INVISIBLE_CHARS.contains(&zero_width_space));

    // Create a string with 50% invisible characters (alternating normal and invisible)
    let mut test_string = String::new();
    for _ in 0..10 {
        test_string.push('a');
        test_string.push(zero_width_space);
    }

    // Count the number of characters (not bytes)
    let char_count = test_string.chars().count();

    // Count invisible characters
    let mut invis_chars_found = 0.0;
    for char in test_string.chars() {
        if INVISIBLE_CHARS
            .iter()
            .any(|invis_chars| *invis_chars == char)
        {
            invis_chars_found += 1.0;
        }
    }

    // Calculate percentage based on character count, not byte length
    let invis_char_percentage = invis_chars_found / char_count as f64;

    // Should be 50%
    assert_eq!(invis_char_percentage, 0.5);
}

/// Test with a string that has no invisible characters except spaces
/// Note: Spaces are considered invisible characters in our implementation
#[test]
fn test_no_invisible_characters() {
    // This string has no invisible characters except spaces
    let test_string = "This is a normal string with no invisible characters.";

    // Count invisible characters
    let mut invis_chars_found = 0.0;
    for char in test_string.chars() {
        if INVISIBLE_CHARS
            .iter()
            .any(|invis_chars| *invis_chars == char)
        {
            invis_chars_found += 1.0;
        }
    }

    // Calculate percentage based on character count, not byte length
    let char_count = test_string.chars().count();
    let invis_char_percentage = invis_chars_found / char_count as f64;

    // Count spaces
    let space_count = test_string.chars().filter(|c| *c == ' ').count() as f64;
    let expected_percentage = space_count / char_count as f64;

    // The invisible characters should be exactly the spaces
    assert_eq!(invis_char_percentage, expected_percentage);

    // Verify that spaces are indeed counted as invisible
    assert!(INVISIBLE_CHARS.contains(&' '));
}

/// Test with a string that has spaces (which are considered invisible)
#[test]
fn test_spaces_as_invisible_characters() {
    let test_string = "This string has spaces.";

    // Count invisible characters
    let mut invis_chars_found = 0.0;
    for char in test_string.chars() {
        if INVISIBLE_CHARS
            .iter()
            .any(|invis_chars| *invis_chars == char)
        {
            invis_chars_found += 1.0;
        }
    }

    // Calculate percentage - should be the number of spaces divided by the total length
    let space_count = test_string.chars().filter(|c| *c == ' ').count() as f64;
    let expected_percentage = space_count / test_string.len() as f64;
    let invis_char_percentage = invis_chars_found / test_string.len() as f64;

    assert_eq!(invis_char_percentage, expected_percentage);
}
