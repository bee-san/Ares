# Sensitivity Levels in Gibberish Detection

## Overview

Ares uses the `gibberish_or_not` library to detect whether decoded text is meaningful English. This library provides three sensitivity levels to fine-tune gibberish detection:

### Low Sensitivity
- Most strict classification
- Requires very high confidence to classify text as English
- Best for detecting texts that appear English-like but are actually gibberish
- Used by classical ciphers like Caesar cipher that produce more English-like results

### Medium Sensitivity (Default)
- Balanced approach for general use
- Combines dictionary and n-gram analysis
- Default mode suitable for most applications
- Used by most decoders in Ares

### High Sensitivity
- Most lenient classification
- Favors classifying text as English
- Best when input is mostly gibberish and any English-like patterns are significant

## Implementation in Ares

In Ares, different decoders use different sensitivity levels based on their characteristics:

1. **Caesar Cipher**: Uses Low sensitivity because classical ciphers often produce text that can appear English-like even when the shift is incorrect.

2. **Other Decoders**: Use Medium sensitivity by default, which provides a balanced approach for most types of encoded text.

## Customizing Sensitivity

Decoders can override the default sensitivity level when needed. The `CheckerTypes` enum provides a `with_sensitivity` method that allows changing the sensitivity level:

```rust
// Example: Using a checker with a custom sensitivity level
let checker_with_sensitivity = checker.with_sensitivity(Sensitivity::High);
let result = checker_with_sensitivity.check(text);
```

## Technical Details

The sensitivity level affects the thresholds used for n-gram analysis and dictionary checks:

- **Low Sensitivity**: Stricter thresholds, requiring more evidence to classify text as English
- **Medium Sensitivity**: Balanced thresholds suitable for most applications
- **High Sensitivity**: Lenient thresholds, more likely to classify text as English

For more details on how the sensitivity levels work, see the [gibberish_or_not documentation](https://crates.io/crates/gibberish-or-not).