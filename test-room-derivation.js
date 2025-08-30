#!/usr/bin/env node

// Quick test to verify room ID generation
// Run with: node test-room-derivation.js

const crypto = require('crypto');

function roomIdFromCode(code) {
    // Strip hyphens
    const cleanCode = code.replace(/-/g, '');
    
    // Hash with domain separator
    const hash = crypto.createHash('sha256');
    hash.update('justcall-v1|');
    hash.update(cleanCode);
    const hashResult = hash.digest();
    
    // Convert to base32 (simplified version for testing)
    const base32Chars = 'abcdefghijklmnopqrstuvwxyz234567';
    let encoded = '';
    
    // Take first 10 bytes (80 bits) and encode
    for (let i = 0; i < 10; i++) {
        const byte = hashResult[i];
        encoded += base32Chars[byte % 32];
        encoded += base32Chars[Math.floor(byte / 32) % 32];
    }
    
    return `jc-${encoded.slice(0, 16)}`;
}

// Test with your code
const testCode = '7aap-ofsd-vxrb-nu4c-ei4z';
const roomId = roomIdFromCode(testCode);

console.log('Testing room derivation:');
console.log('Code:', testCode);
console.log('Room ID:', roomId);
console.log('Jitsi URL:', `https://meet.jit.si/${roomId}`);
console.log('\nBoth devices should generate the same room ID from the same code.');
console.log('If they don\'t connect, check:');
console.log('1. Network/firewall - Can you access meet.jit.si?');
console.log('2. Browser console for errors');
console.log('3. Application logs (check terminal where npm run tauri dev is running)');
