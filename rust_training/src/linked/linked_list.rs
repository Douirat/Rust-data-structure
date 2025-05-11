// Create a struct to represent the node in q linked list:
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>, // either some (box<node>) or none:
}

// Associate functions to my Node struct:
impl Node {
    pub fn new(data: i32) -> Box<Node>{
        Box::new(Node{data, next: none})
    }
}