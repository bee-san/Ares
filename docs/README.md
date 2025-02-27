# Ares Documentation

Welcome to the Ares documentation! This repository contains comprehensive documentation for Ares, the next-generation automatic decoding and cracking tool.

## Table of Contents

### General Documentation

- [Ares Overview](ares_overview.md) - A high-level overview of Ares, its features, and capabilities
- [Using Ares](using_ares.md) - A comprehensive guide on how to use Ares, with examples and common use cases

### Technical Documentation

- [Ares Architecture](ares_architecture.md) - Detailed explanation of Ares's internal architecture and components
- [Plaintext Identification](plaintext_identification.md) - How Ares identifies plaintext and determines when decoding is successful

### Feature-Specific Documentation

- [Invisible Characters Detection](invisible_characters.md) - Information about Ares's capability to detect and handle invisible Unicode characters
- [Package Managers](package-managers.md) - Guidelines for packaging Ares for different package managers

## About Ares

Ares is the next generation of decoding tools, built by the same people that brought you [Ciphey](https://github.com/ciphey/ciphey). It's designed to automatically detect and decode various types of encoded or encrypted text, including (but not limited to) Base64, Hexadecimal, Caesar cipher, ROT13, URL encoding, and many more.

Key features include:

- Significantly faster performance (up to 700% faster than Ciphey)
- Library-first architecture for easy integration
- Advanced search algorithms for efficient decoding
- Built-in timeout mechanism
- Comprehensive documentation and testing
- Support for multi-level encodings

## Getting Started

The quickest way to get started with Ares is to install it via Cargo:

```bash
cargo install project_ares
```

Then use it with the `ares` command:

```bash
ares "your encoded text here"
```

For more detailed instructions, see the [Using Ares](using_ares.md) guide.

## Contributing

Contributions to Ares are welcome! Whether it's adding new decoders, improving existing ones, enhancing documentation, or fixing bugs, your help is appreciated. Check the [GitHub repository](https://github.com/bee-san/Ares) for more information on how to contribute.

## Additional Resources

- [GitHub Repository](https://github.com/bee-san/Ares)
- [Discord Server](http://discord.skerritt.blog)
- [Blog Post: Introducing Ares](https://skerritt.blog/introducing-ares/)
- [Ciphey2 Documentation](https://broadleaf-angora-7db.notion.site/Ciphey2-32d5eea5d38b40c5b95a9442b4425710)