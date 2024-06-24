use std::cmp::Ordering;

#[derive(Debug)]
pub struct Node {
    pub ch: Option<char>,
    pub freq: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl Eq for Node {}
