/// Room ID derivation from pairing codes
/// What: Convert pairing codes to deterministic Jitsi room names
/// Why: Both partners need to join the exact same room without coordination
/// Used by:
///   - CallController::join() (Phase 5.1)
///   - ConferenceWindow::open() (Phase 4.2)
///   - Tests: integration tests, room switching tests

use sha2::{Digest, Sha256};
use data_encoding::BASE32_NOPAD;

/// room_id_from_code(code)
/// What: Derives a deterministic room ID from a pairing code
/// Why: Same code must always produce same room for both partners to meet
/// Contract:
///   - Input: pairing code with or without hyphens
///   - Output: "jc-" + 16 lowercase base32 chars (e.g., "jc-abc123def456gh")
///   - Deterministic: same input always gives same output
/// Used by:
///   - CallController::join() - derives room when user presses hotkey
///   - InviteHandler::preview_room() - shows room ID before accepting invite
///   - Tests: pairing_flow_test, room_consistency_test
/// Calls:
///   - sha2::Sha256 - cryptographic hash
///   - data_encoding::BASE32_NOPAD - encoding
/// Change notes:
///   - If changing prefix or length, update Jitsi config to match
///   - Format must stay consistent or existing pairs can't connect
pub fn room_id_from_code(code: &str) -> String {
    // Strip hyphens if present (handle both formats)
    let clean_code = code.replace('-', "");
    
    // Hash with domain separation to prevent collisions with other apps
    let mut hasher = Sha256::new();
    hasher.update(b"justcall-v1|"); // Domain separator
    hasher.update(clean_code.as_bytes());
    let hash_result = hasher.finalize();
    
    // Encode to base32 and take first 16 chars (~80 bits)
    let encoded = BASE32_NOPAD.encode(&hash_result).to_lowercase();
    
    // Format: "jc-" prefix + 16 chars
    // Jitsi accepts alphanumeric room names, we use base32 subset
    format!("jc-{}", &encoded[..16])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_room_id() {
        let code = "test-code-1234-5678-abcd";
        let room1 = room_id_from_code(code);
        let room2 = room_id_from_code(code);
        assert_eq!(room1, room2, "Same code must produce same room");
    }
    
    #[test]
    fn test_different_codes_different_rooms() {
        let room1 = room_id_from_code("code-one");
        let room2 = room_id_from_code("code-two");
        assert_ne!(room1, room2, "Different codes must produce different rooms");
    }
    
    #[test]
    fn test_room_format() {
        let room = room_id_from_code("test");
        assert!(room.starts_with("jc-"), "Room must start with 'jc-'");
        assert_eq!(room.len(), 19, "Room should be 'jc-' + 16 chars");
        
        // Verify only lowercase alphanumeric after prefix
        let suffix = &room[3..];
        assert!(suffix.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "Room ID should only contain lowercase alphanumeric");
    }
    
    #[test]
    fn test_hyphen_stripping() {
        let with_hyphens = room_id_from_code("abcd-efgh-ijkl-mnop-qrst");
        let without_hyphens = room_id_from_code("abcdefghijklmnopqrst");
        assert_eq!(with_hyphens, without_hyphens, 
                   "Code with/without hyphens should produce same room");
    }
    
    #[test]
    fn test_integration_with_generated_code() {
        // Test with actual generated code from phase 1.1
        use crate::core::generate_code_base32_100b;
        
        let code = generate_code_base32_100b();
        let room = room_id_from_code(&code);
        
        // Should work and produce valid room
        assert!(room.starts_with("jc-"));
        assert_eq!(room.len(), 19);
        
        // Same code twice = same room
        let room2 = room_id_from_code(&code);
        assert_eq!(room, room2);
    }
    
    // Edge case tests
    
    #[test]
    fn test_empty_code() {
        let room = room_id_from_code("");
        assert!(room.starts_with("jc-"));
        assert_eq!(room.len(), 19);
        // Empty input should still produce valid room
    }
    
    #[test]
    fn test_very_long_code() {
        let long_code = "a".repeat(10000);
        let room = room_id_from_code(&long_code);
        assert!(room.starts_with("jc-"));
        assert_eq!(room.len(), 19);
        // Should handle long inputs without panic
    }
    
    #[test]
    fn test_special_characters() {
        let test_cases = vec![
            "!@#$%^&*()_+=",
            "Hello World!",
            "code with spaces",
            "unicode-🦀-rust",
            "\n\r\t",
            "<script>alert('xss')</script>",
            "'; DROP TABLE rooms; --",
        ];
        
        for test_code in test_cases {
            let room = room_id_from_code(test_code);
            assert!(room.starts_with("jc-"), "Failed for: {}", test_code);
            assert_eq!(room.len(), 19, "Failed for: {}", test_code);
            
            // Only base32 chars in output
            let suffix = &room[3..];
            assert!(
                suffix.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "Invalid chars in room for input: {}", test_code
            );
        }
    }
    
    #[test]
    fn test_case_sensitivity() {
        let room_lower = room_id_from_code("test-code");
        let room_upper = room_id_from_code("TEST-CODE");
        let room_mixed = room_id_from_code("TeSt-CoDe");
        
        // Different cases should produce different rooms
        assert_ne!(room_lower, room_upper);
        assert_ne!(room_lower, room_mixed);
        assert_ne!(room_upper, room_mixed);
    }
    
    #[test]
    fn test_whitespace_handling() {
        let room1 = room_id_from_code("test code");
        let room2 = room_id_from_code("test  code");  // double space
        let room3 = room_id_from_code(" test code");  // leading space
        let room4 = room_id_from_code("test code ");  // trailing space
        let room5 = room_id_from_code("test\tcode");  // tab
        
        // All should be different
        let rooms = vec![&room1, &room2, &room3, &room4, &room5];
        for i in 0..rooms.len() {
            for j in i+1..rooms.len() {
                assert_ne!(rooms[i], rooms[j], 
                          "Whitespace handling failed: {} == {}", i, j);
            }
        }
    }
    
    #[test]
    fn test_concurrent_derivation() {
        use std::thread;
        
        let code = "test-concurrent-code";
        let mut handles = vec![];
        
        // 10 threads deriving same room
        for _ in 0..10 {
            let code_copy = code.to_string();
            let handle = thread::spawn(move || {
                room_id_from_code(&code_copy)
            });
            handles.push(handle);
        }
        
        // Collect results
        let results: Vec<String> = handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        
        // All should be identical
        let first = &results[0];
        for room in &results {
            assert_eq!(room, first, "Concurrent derivation produced different results");
        }
    }
    
    #[test]
    fn test_similar_codes_different_rooms() {
        // Similar codes should produce very different rooms
        let room1 = room_id_from_code("test1");
        let room2 = room_id_from_code("test2");
        
        // Rooms should be different beyond just last char
        let diff_chars = room1.chars().zip(room2.chars())
            .filter(|(a, b)| a != b)
            .count();
        
        assert!(diff_chars > 5, 
                "Similar inputs produced too-similar outputs: {} vs {}", room1, room2);
    }
    
    #[test]
    fn test_performance_with_various_inputs() {
        use std::time::Instant;
        
        let long_input = "a".repeat(1000);
        let inputs = vec![
            "",
            "a",
            long_input.as_str(),
            "test-code-1234-5678-abcd",
            "🦀🔥💻🎉",
        ];
        
        for input in inputs {
            let start = Instant::now();
            let _ = room_id_from_code(input);
            let duration = start.elapsed();
            
            // Each derivation should be fast (< 1ms)
            assert!(
                duration.as_micros() < 1000,
                "Slow derivation for input length {}: {:?}", input.len(), duration
            );
        }
    }
}
