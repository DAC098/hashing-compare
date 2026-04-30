use md5::Digest;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Md5(md5::Md5);

#[wasm_bindgen]
impl Md5 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(md5::Md5::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha1(sha1::Sha1);

#[wasm_bindgen]
impl Sha1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha1::Sha1::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha2_256(sha2::Sha256);

#[wasm_bindgen]
impl Sha2_256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha2::Sha256::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha2_384(sha2::Sha384);

#[wasm_bindgen]
impl Sha2_384 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha2::Sha384::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha2_512(sha2::Sha512);

#[wasm_bindgen]
impl Sha2_512 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha2::Sha512::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha3_256(sha3::Sha3_256);

#[wasm_bindgen]
impl Sha3_256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha3::Sha3_256::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha3_384(sha3::Sha3_384);

#[wasm_bindgen]
impl Sha3_384 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha3::Sha3_384::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Sha3_512(sha3::Sha3_512);

#[wasm_bindgen]
impl Sha3_512 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(sha3::Sha3_512::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[wasm_bindgen]
pub struct Blake3(blake3::Hasher);

#[wasm_bindgen]
impl Blake3 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.0.finalize().as_bytes().to_vec()
    }
}
