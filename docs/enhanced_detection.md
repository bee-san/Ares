# Enhanced Plaintext Detection

## Overview

Ciphey offers an optional **Enhanced Plaintext Detection** feature that uses a BERT-based AI model to significantly improve the accuracy of identifying whether decoded text is meaningful plaintext or gibberish. This feature increases detection accuracy by approximately 40% compared to the standard detection method.

## The Model: gibberish-or-not

The enhanced detection uses the [gibberish-or-not](https://github.com/bee-san/gibberish-or-not) Rust library, which provides a BERT (Bidirectional Encoder Representations from Transformers) model fine-tuned specifically for distinguishing English text from gibberish.

### What is BERT?

BERT is a transformer-based machine learning model developed by Google that understands the context of words in a sentence by looking at words bidirectionally (both left-to-right and right-to-left). Unlike traditional approaches that rely on simple dictionary lookups or n-gram analysis, BERT understands semantic relationships and can better recognize text that "makes sense" even if it contains unusual words or phrasing.

### Why Use BERT for Gibberish Detection?

Traditional gibberish detection methods used by ciphey include:

1. **Dictionary Analysis**: Checking if words exist in a dictionary (~370,000 English words)
2. **N-gram Analysis**: Analyzing character sequences (trigrams and quadgrams) against known patterns
3. **Character Transition Probability**: Checking if character pairs follow natural English patterns
4. **Vowel-Consonant Ratio**: Verifying text has a typical vowel-to-consonant balance

These methods work well for clear cases but struggle with:
- Text containing technical jargon or proper nouns not in the dictionary
- Borderline cases that are partially English-like
- Short sentences where statistical methods have insufficient data
- Creative or unusual phrasing

The BERT model addresses these limitations by understanding context and meaning rather than just pattern matching.

## How It Works

### Basic Detection (Default)

Without enhanced detection, ciphey uses a weighted composite score:

```
Score = 0.4 * (English word ratio) 
      + 0.25 * (Character transition probability)
      + 0.15 * (Trigram score)
      + 0.10 * (Quadgram score)
      + 0.10 * (Vowel-consonant ratio)
```

This provides fast detection (~2-15 microseconds) with good accuracy for most cases.

### Enhanced Detection (With BERT)

When enabled, the BERT model provides deeper semantic analysis:
- First run includes model loading (~100ms)
- Subsequent runs: 5-30ms depending on text length
- Memory usage: ~400-500MB (model is memory-mapped)

The model was trained on a large corpus of English text and gibberish samples to learn the subtle patterns that distinguish meaningful text from nonsense.

## Enabling Enhanced Detection

### During First Run

When you first run ciphey, the setup wizard will ask:

```
Would you like to enable Enhanced Plaintext Detection?

This will increase accuracy by around 40%, and you will be asked less 
frequently if something is plaintext or not.

This will download a 500mb AI model.
```

If you choose yes, you'll need to:
1. Create a [HuggingFace account](https://huggingface.co/)
2. Generate a [READ token](https://huggingface.co/settings/tokens)
3. Enter the token when prompted (input is hidden for security)

The model will download to `~/.config/ciphey/models/model.bin` (or equivalent on your platform).

### After Initial Setup

If you skipped enhanced detection during setup, you can enable it later:

```bash
ciphey --enable-enhanced-detection
```

This will prompt you for your HuggingFace token and download the model.

## Performance Comparison

| Metric | Basic Detection | Enhanced Detection |
|--------|----------------|-------------------|
| Accuracy | ~85-90% | ~95%+ |
| Speed (short text) | 2-7 μs | 5-10 ms |
| Speed (long text) | 15-50 μs | 15-30 ms |
| Memory | < 1 MB | ~400-500 MB |
| Requires Internet | No | Only for download |
| False Positive Rate | Higher | Lower |

## When to Use Enhanced Detection

**Recommended for:**
- CTF challenges with unusual or creative flag formats
- Decoding text that may contain technical jargon
- Situations where false positives are costly
- Complex nested encodings where intermediate results may look partially valid

**May not be necessary for:**
- Simple encodings with clear English output
- Time-critical applications where every millisecond counts
- Resource-constrained environments
- Offline environments without prior model download

## Alternatives to BERT-Based Detection

If you cannot or prefer not to use the enhanced BERT detection, ciphey provides several alternatives:

### 1. Custom Wordlist Checker

Provide your own wordlist of expected plaintext values:

```bash
ciphey -t "encoded_text" --wordlist /path/to/wordlist.txt
```

The wordlist checker performs exact matches against your list, useful when you know the expected output.

### 2. Regex Pattern Matching

If you know the format of the expected plaintext (e.g., a CTF flag format):

```bash
ciphey -t "encoded_text" --regex "flag\{.*\}"
```

This bypasses language detection entirely and matches against your pattern.

### 3. Sensitivity Adjustment

The basic detection supports three sensitivity levels:
- **Low**: Strictest - requires high confidence (good for cipher decoding)
- **Medium**: Balanced - suitable for most use cases (default)
- **High**: Most lenient - catches borderline cases

Different decoders automatically use appropriate sensitivity levels based on their characteristics.

### 4. Human Checker

For maximum accuracy with human oversight:

```bash
ciphey -t "encoded_text"
```

During the first-run setup, choose option 1 to be asked about each potential plaintext. This ensures no false positives but requires user interaction.

### 5. Top Results Mode

During setup, choose option 2 to have ciphey collect all potential plaintexts and present them at the end. This lets you manually review all candidates without interrupting the search.

## Technical Details

### Model Location

The model is stored at:
- Linux/macOS: `~/.config/ciphey/models/model.bin`
- Windows: `%APPDATA%\ciphey\models\model.bin`

### Integration in Code

Enhanced detection is integrated into the English checker (`src/checkers/english.rs`):

```rust
fn check(&self, text: &str) -> CheckResult {
    let is_enhanced = config.enhanced_detection;
    
    let mut result = CheckResult {
        is_identified: if is_enhanced {
            !is_gibberish(&text, Sensitivity::High)
        } else {
            !is_gibberish(&text, self.sensitivity)
        },
        // ...
    };
    // ...
}
```

### HuggingFace Token Security

- The token is only used during model download
- It is never stored to disk
- It is read using `rpassword` which hides input from the terminal
- After download, the token is discarded from memory

## Troubleshooting

### Model Download Fails

1. Verify your HuggingFace token is valid and has READ permissions
2. Check your internet connection
3. Ensure you have ~500MB of free disk space
4. Try running with `--enable-enhanced-detection` again

### Enhanced Detection Not Working

1. Check that `enhanced_detection = true` in your config file
2. Verify the model file exists at the expected path
3. Check ciphey's output for warning messages about missing model

### Performance Issues

If enhanced detection is too slow:
1. Consider using basic detection for time-critical tasks
2. Use regex patterns if you know the expected format
3. The model is memory-mapped, so ensure adequate RAM

## Further Reading

- [gibberish-or-not GitHub Repository](https://github.com/bee-san/gibberish-or-not)
- [BERT Paper (Devlin et al., 2018)](https://arxiv.org/abs/1810.04805)
- [HuggingFace Transformers](https://huggingface.co/docs/transformers)
- [Plaintext Identification in Ciphey](./plaintext_identification.md)
