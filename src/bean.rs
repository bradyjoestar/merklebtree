use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Item {
    pub key: u32,
    pub value: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key + self.value == other.key + other.value
    }
}
impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key + self.value).cmp(&(other.key + other.value))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
