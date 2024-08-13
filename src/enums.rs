enum Truety {
    Some(bool),
    False
}

impl Default for Truety {
    fn default() -> Self {
        Self::False
    }
}
