pub trait IdGenerator {
    fn next_id(&mut self) -> String;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NumberIdGenerator(pub usize);

impl IdGenerator for NumberIdGenerator {
    fn next_id(&mut self) -> String {
        self.0 += 1;
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_ids() {
        let mut id_gen = NumberIdGenerator(0);

        assert_eq!(id_gen.next_id(), "1");
        assert_eq!(id_gen.next_id(), "2");
        assert_eq!(id_gen.next_id(), "3");
    }
}
