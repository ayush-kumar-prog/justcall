/// Cryptographic utilities for secure code generation
/// What: Generate secure pairing codes for targets
/// Why: Need unpredictable, high-entropy codes to prevent collisions
/// Used by: 
///   - SettingsStore::create_target() (Phase 2.2)
///   - OnboardingWizard::generate_code() (Phase 8.3) 
///   - InviteSystem::create_invite() (Phase 7.2)

use rand::RngCore;
use data_encoding::BASE32_NOPAD;

/// generate_code_base32_100b()
/// What: Creates a cryptographically secure 100-bit code formatted for humans
/// Why: Pairing codes must be unguessable but shareable; 100 bits prevents brute force
/// Contract:
///   - Returns: 24-char string formatted as "xxxx-xxxx-xxxx-xxxx-xxxx"
///   - Charset: lowercase base32 (a-z, 2-7) - no confusing 0/O, 1/I/l
///   - Entropy: ~100 bits (16 bytes encoded, take 20 chars)
/// Used by:
///   - SettingsStore::create_target() - when user adds new partner/group
///   - InviteHandler::generate_invite() - creating sharable links
///   - Tests: settings_integration_test, invite_flow_test
/// Calls:
///   - rand::rngs::OsRng - system CSPRNG
///   - data_encoding::BASE32_NOPAD - RFC 4648 encoding
/// Change notes:
///   - If changing format, update room_id_from_code() parser
///   - If changing length, update validation in SettingsStore
///   - Format MUST stay consistent or existing pairs break
pub fn generate_code_base32_100b() -> String {
    // Use OS random source - best entropy available
    // 16 bytes = 128 bits, we'll use first 100 bits (20 base32 chars)
    let mut raw_bytes = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut raw_bytes);
    
    // BASE32_NOPAD uses A-Z2-7, we lowercase for better UX
    // No padding chars (=) to keep it clean
    let encoded = BASE32_NOPAD.encode(&raw_bytes).to_lowercase();
    
    // Take exactly 20 chars (100 bits of entropy)
    // Each base32 char encodes 5 bits: 20 chars * 5 = 100 bits
    let code_chars: String = encoded.chars().take(20).collect();
    
    // Format with hyphens every 4 chars for readability
    // Like: "f7rx-kq3m-29p8-z4nh-td8w"
    format!(
        "{}-{}-{}-{}-{}",
        &code_chars[0..4],
        &code_chars[4..8],
        &code_chars[8..12],
        &code_chars[12..16],
        &code_chars[16..20]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    /// Test: Codes are unique (no collisions in reasonable sample)
    /// Why: Verify CSPRNG is working and we have enough entropy
    #[test]
    fn test_code_uniqueness() {
        let mut generated_codes = HashSet::new();
        const SAMPLE_SIZE: usize = 10_000;
        
        for _ in 0..SAMPLE_SIZE {
            let code = generate_code_base32_100b();
            let was_new = generated_codes.insert(code.clone());
            assert!(was_new, "Duplicate code generated: {}", code);
        }
        
        // Sanity check we actually generated the requested amount
        assert_eq!(generated_codes.len(), SAMPLE_SIZE);
    }
    
    /// Test: Code format matches specification
    /// Why: Other components parse this exact format
    #[test]
    fn test_code_format() {
        let code = generate_code_base32_100b();
        
        // Total length: 20 chars + 4 hyphens = 24
        assert_eq!(code.len(), 24, "Code length mismatch: {}", code);
        
        // Check hyphen positions (indices 4, 9, 14, 19)
        assert_eq!(&code[4..5], "-", "Missing hyphen at position 4");
        assert_eq!(&code[9..10], "-", "Missing hyphen at position 9");
        assert_eq!(&code[14..15], "-", "Missing hyphen at position 14");
        assert_eq!(&code[19..20], "-", "Missing hyphen at position 19");
        
        // Verify only valid base32 lowercase chars and hyphens
        for ch in code.chars() {
            assert!(
                ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-',
                "Invalid character '{}' in code", ch
            );
            
            // base32 specifically excludes 0, 1, 8, 9
            if ch.is_ascii_digit() {
                assert!(
                    "234567".contains(ch),
                    "Invalid base32 digit '{}' in code", ch
                );
            }
        }
    }
    
    /// Test: Code has sufficient entropy
    /// Why: Security depends on unguessability
    #[test]
    fn test_code_entropy() {
        // Generate multiple codes and check character distribution
        let mut char_frequency = std::collections::HashMap::new();
        const SAMPLE_SIZE: usize = 1000;
        
        for _ in 0..SAMPLE_SIZE {
            let code = generate_code_base32_100b();
            // Count only the actual code chars, not hyphens
            for ch in code.chars().filter(|&c| c != '-') {
                *char_frequency.entry(ch).or_insert(0) += 1;
            }
        }
        
        // With good randomness, each char should appear roughly equally
        // base32 has 32 possible chars, we expect ~625 occurrences each
        // (20 chars per code * 1000 codes / 32 possible chars)
        let expected_avg = (20 * SAMPLE_SIZE) / 32;
        let tolerance = expected_avg / 4; // Allow 25% deviation
        
        for (ch, count) in char_frequency {
            assert!(
                count > expected_avg - tolerance && count < expected_avg + tolerance,
                "Character '{}' appeared {} times, expected ~{}", ch, count, expected_avg
            );
        }
    }
    
    /// Test: Codes work with room derivation expectations
    /// Why: room_id_from_code() will strip hyphens and hash the result
    #[test]
    fn test_code_stripping_and_validation() {
        let code = generate_code_base32_100b();
        
        // Simulate what room_id_from_code will do
        let stripped = code.replace('-', "");
        assert_eq!(stripped.len(), 20, "Stripped code should be 20 chars");
        
        // Verify only valid base32 chars (lowercase)
        for ch in stripped.chars() {
            assert!(
                ch.is_ascii_lowercase() || "234567".contains(ch),
                "Invalid character '{}' in stripped code", ch
            );
        }
        
        // The code doesn't need to decode back to bytes - it's just an identifier
        // room_id_from_code() will hash it as-is, not decode it
    }
    
    /// Test: Performance is acceptable
    /// Why: Code generation happens in UI paths
    #[test]
    fn test_generation_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = generate_code_base32_100b();
        }
        let duration = start.elapsed();
        
        // Should generate 1000 codes in under 100ms
        // Typical perf: ~10ms on modern hardware
        assert!(
            duration.as_millis() < 100,
            "Code generation too slow: {:?} for 1000 codes", duration
        );
    }
}
