# Change: Add Vigenère Cipher Decoder

## Purpose
Implement a Vigenère cipher decoder to expand Ares' classical cipher decoding capabilities. This decoder will automatically detect and break Vigenère encrypted text without requiring a key, making it valuable for cryptanalysis and historical cipher decoding.

## Trade-offs
### Advantages
- Implements sophisticated frequency analysis for automated key length detection
- Uses Index of Coincidence (IoC) for reliable key length determination
- Employs statistical analysis to break the cipher without requiring the key
- Handles both known-key and unknown-key scenarios

### Disadvantages
- Computationally more intensive than simple substitution ciphers
- May produce false positives with very short texts
- Effectiveness depends on text length and language characteristics

## Technical Implementation
- Added Vigenère decoder module with key length detection using IoC
- Implemented frequency analysis for automated key discovery
- Added comprehensive test suite with example ciphertexts
- Integrated with Ares' existing decoder infrastructure
- Popularity score set to 0.8 reflecting its historical significance

## Future Improvements
- Add support for multiple languages beyond English
- Implement parallel processing for faster key space exploration
- Add option to specify known key length or partial key
- Enhance accuracy for very short ciphertexts
- Add support for variant ciphers (Beaufort, Gronsfeld) 