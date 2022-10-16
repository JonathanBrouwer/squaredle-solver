use std::mem::swap;

pub struct Trie {
    pub end: bool,
    pub bytes: &'static [u8],
    pub children: [Option<Box<Trie>>; 26]
}

impl Trie {
    pub fn new() -> Self {
        Self {
            end: false,
            bytes: &[],
            children: [(); 26].map(|_| None),
        }
    }

    fn from_bytes(bytes: &'static [u8]) -> Self {
        Self {
            end: true,
            bytes,
            children: [(); 26].map(|_| None),
        }
    }

    /// Insert the bytes into this Trie
    pub fn insert(&mut self, bytes: &'static [u8]) {
        // If we don't need to split this node, don't
        if bytes.starts_with(self.bytes) {
            let subbytes = &bytes[self.bytes.len()..];

            // Because we insert words in alphabetical order, we can never get a word before its prefix
            let child = &mut self.children[(subbytes[0] - b'a') as usize];
            if let Some(child) = child {
                child.insert(subbytes);
            } else {
                *child = Some(Box::new(Self::from_bytes(subbytes)));
            }
        } else {
            // We need to split this node at this byte
            let prefix = bytes.iter().zip(self.bytes.iter()).take_while(|(b1, b2)| **b1 == **b2).count();

            // Swap self with a new trie that contains the prefix
            let mut old_self = Trie {
                end: false,
                bytes: &self.bytes[..prefix],
                children: [(); 26].map(|_| None),
            };
            swap(self, &mut old_self);

            // Prepare second node
            old_self.bytes = &old_self.bytes[prefix..];

            // Add second node to first node
            let idx = old_self.bytes[0] - b'a';
            self.children[idx as usize] = Some(Box::new(old_self));

            // Add word into self
            self.insert(&bytes);
        }
    }
}