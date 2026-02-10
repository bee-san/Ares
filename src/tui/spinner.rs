//! Loading spinner and cryptography quotes for the TUI.
//!
//! This module provides spinner animation frames and a collection of funny/interesting
//! cryptography-related quotes to display during decoding operations. Quotes are
//! randomized each time to keep users entertained with diverse cryptography facts.

use rand::Rng;

/// Enhanced braille spinner frames for more visible animation (fuller dots).
pub const ENHANCED_SPINNER_FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// Collection of funny and interesting cryptography-related quotes.
pub const QUOTES: &[&str] = &[
    "The only truly secure system is one that is powered off. - Gene Spafford",
    "Cryptography is typically bypassed, not penetrated. - Adi Shamir",
    "Base64 isn't encryption, it's encoding. Now you know!",
    "Brute forcing 26 keys since 44 BC...",
    "Fun fact: The Caesar cipher is named after Julius Caesar!",
    "Security is a process, not a product. - Bruce Schneier",
    "There are two types of encryption: one that will prevent your sister from reading your diary, and one that will prevent your government. - Bruce Schneier",
    "The Enigma machine had 158,962,555,217,826,360,000 possible settings!",
    "ROT13 twice for extra security! (Just kidding, please don't.)",
    "If you think cryptography can solve your problem, you don't understand cryptography. - Peter Neumann",
    "Fun fact: The word 'cipher' comes from the Arabic 'sifr' meaning zero!",
    "Attempting to decode... or maybe just staring at random bytes.",
    "In cryptography, we trust math, not people.",
    "The Vigenere cipher was once called 'le chiffre indechiffrable' (the unbreakable cipher). We broke it.",
    "British triumph: Bletchley Park broke the Enigma cipher, helping win WWII!",
    "Alan Turing: British mathematician who cracked Enigma and invented the Turing machine.",
    "Fun fact: The Bombe machine at Bletchley Park tested 17,576 rotor combinations per second!",
    "British history: The Government Code and Cypher School operated from 1919, one of the oldest crypto agencies.",
    "Did you know? Over 10,000 people worked at Bletchley Park during WWII, mostly kept secret for decades!",
    "Colossus at Bletchley Park (1943): One of the first programmable computers, used to break the Lorenz cipher.",
    "Fun fact: The British 'Playfair cipher' (1854) was used in military communications for over 100 years!",
    "Trivia: The word 'decryption' became common after British codebreakers needed a verb for their work!",
    "British legend: Tommy Flowers designed Colossus, a computer that predates most other electronic computers.",
    "GCHQ secret: British researchers (Ellis, Cocks, Williamson) invented public-key cryptography in the 1970s, decades before it was publicly credited to Diffie, Hellman & Merkle. Kept classified until 1997!",
    "Clifford Cocks at GCHQ (1973) discovered RSA encryption using prime factorization—years before Rivest, Shamir & Adleman independently invented it in 1977.",
    "Fun fact: Public-key cryptography's true potential wasn't realized until the Web was invented. Tim Berners-Lee designed the open internet at CERN (1989), enabling the crypto revolution!",
    "The first recorded computer virus, 'Creeper' (1971), left the message: 'I'M THE CREEPER, CATCH ME IF YOU CAN!'",
    "In 1988, the Morris Worm crashed about 10% of the internet in a single day. The creator claimed it was a prank.",
    "Your password 'Password123!' is weak. A modern computer can crack it in seconds.",
    "The 2013 Target breach exposed 40 million credit card numbers through an air conditioning company's login.",
    "SQL Injection attacks allow hackers to access entire databases with a simple quote mark.",
    "Yahoo's 2013 breach exposed 3 billion user accounts. They delayed notification for three years.",
    "The Equifax breach (2017) exposed 147 million people's social security numbers.",
    "Phishing emails fool about 12% of recipients into handing over passwords.",
    "Default passwords like 'admin/admin' were found on 68,000 IoT devices connected to the internet.",
    "The Heartbleed bug (2014) allowed hackers to steal data from millions of computers without leaving traces.",
    "Bitcoin exchanges have been hacked for billions of dollars. One hacker stole 650,000 bitcoins.",
    "NotPetya ransomware (2017) caused an estimated $10 billion in damages globally.",
    "Ransomware operators now earn billions annually. Crime is surprisingly profitable.",
    "Two-factor authentication can be bypassed through social engineering. Hackers just call and ask.",
];

/// Returns a random starting index into the [`QUOTES`] array.
///
/// Used to pick a random starting quote when entering the loading state,
/// so each decoding session begins with a different quote.
pub fn random_quote_index() -> usize {
    rand::thread_rng().gen_range(0..QUOTES.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_count() {
        assert_eq!(QUOTES.len(), 40);
    }

    #[test]
    fn test_enhanced_spinner_frame_count() {
        assert_eq!(ENHANCED_SPINNER_FRAMES.len(), 8);
    }

    #[test]
    fn test_random_quote_index_in_range() {
        for _ in 0..100 {
            let idx = random_quote_index();
            assert!(idx < QUOTES.len());
        }
    }
}
