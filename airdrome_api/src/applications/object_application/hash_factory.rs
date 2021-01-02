use super::hash_model::Hash;

pub fn create_hash(hash: &str) -> Result<Hash, &'static str> {
    if hash.is_empty() {
        Err("Hash can't be empty")
    } else if hash.chars().count() > 40 {
        Err("Hash must be less than 40 chracters")
    } else {
        Hash::new(hash)
    }
}
