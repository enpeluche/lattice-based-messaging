pub type Key256 = [u8; 32];
pub type Nonce96 = [u8; 12];


pub const KEY256: Key256 = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

pub const NONCE96: Nonce96 = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00];

pub struct ChaCha20Context {
    state: [u32; 4*4],
    keystream: [u32; 4*4],
    idx: usize
}

pub fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = state[d].rotate_left(16);
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = state[b].rotate_left(12);
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = state[d].rotate_left(8);
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = state[b].rotate_left(7);
}

pub fn double_round(state: &mut [u32; 16]) {
    quarter_round(state, 0, 4,  8, 12); // Column 0
	quarter_round(state, 1, 5,  9, 13); // Column 1
	quarter_round(state, 2, 6, 10, 14); // Column 2
	quarter_round(state, 3, 7, 11, 15); // Column 3

	quarter_round(state, 0, 5, 10, 15); // Diagonal 1 (main diagonal)
	quarter_round(state, 1, 6, 11, 12); // Diagonal 2
	quarter_round(state, 2, 7,  8, 13); // Diagonal 3
	quarter_round(state, 3, 4,  9, 14); // Diagonal 4
}

const CHACHA20_CONSTANT: &[u8; 16] = b"expand 32-byte k";

impl ChaCha20Context {
    pub fn new(key: &Key256, nonce: &Nonce96, count: u32) -> Self {
        let mut state = [0u32; 16];

        state[0] = u32::from_le_bytes(CHACHA20_CONSTANT[0..4].try_into().unwrap());
        state[1] = u32::from_le_bytes(CHACHA20_CONSTANT[4..8].try_into().unwrap());
        state[2] = u32::from_le_bytes(CHACHA20_CONSTANT[8..12].try_into().unwrap());
        state[3] = u32::from_le_bytes(CHACHA20_CONSTANT[12..16].try_into().unwrap());

        state[4] = u32::from_le_bytes(key[0..4].try_into().unwrap());
        state[5] = u32::from_le_bytes(key[4..8].try_into().unwrap());
        state[6] = u32::from_le_bytes(key[8..12].try_into().unwrap());
        state[7] = u32::from_le_bytes(key[12..16].try_into().unwrap());
        state[8] = u32::from_le_bytes(key[16..20].try_into().unwrap());
        state[9] = u32::from_le_bytes(key[20..24].try_into().unwrap());
        state[10] = u32::from_le_bytes(key[24..28].try_into().unwrap());
        state[11] = u32::from_le_bytes(key[28..32].try_into().unwrap());

        state[12] = count;

        state[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        state[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());

        Self {
            state,
            keystream: [0u32; 16],
            idx: 0,
        }
    }

    pub fn generate_block(&mut self) {
        let mut working_state = self.state;

        for _ in 0..10 {
            double_round(&mut working_state);
        }

        for i in 0..16 {
            self.keystream[i] = working_state[i].wrapping_add(self.state[i]);
        }
    }
    
    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            if self.idx % 64 == 0 {
                self.generate_block();
                
                self.state[12] = self.state[12].wrapping_add(1);
                
                if self.state[12] == 0 {
                    self.state[13] = self.state[13].wrapping_add(1);
                    assert!(self.state[13] != 0, "Erreur fatale : Compteur de nonce épuisé !");
                }
                self.idx = 0;
            }

            let word_idx = self.idx / 4;
            let byte_idx = self.idx % 4;
            let key_byte = self.keystream[word_idx].to_le_bytes()[byte_idx];

            *byte ^= key_byte;
            
            self.idx += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20_hello_world() {
        let mut message = b"Hello world ?".to_vec();
        println!("Message original : {:?}", String::from_utf8_lossy(&message));

        let mut ctx_encrypt = ChaCha20Context::new(&KEY256, &NONCE96, 0);
        
        ctx_encrypt.apply_keystream(&mut message);
        
        print!("Message chiffré (Hex) : ");
        for byte in &message {
            print!("{:02x}", byte);
        }
        println!();

        let mut ctx_decrypt = ChaCha20Context::new(&KEY256, &NONCE96, 0);
        
        ctx_decrypt.apply_keystream(&mut message);

        let message_dechiffre = String::from_utf8(message).expect("Texte invalide");
        println!("Message déchiffré : {}", message_dechiffre);

        assert_eq!(message_dechiffre, "Hello world ?");
    }
}