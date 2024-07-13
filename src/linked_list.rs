pub mod LinkedList {
    #[derive(Debug)]
    pub struct Node {
        pub data: i32,
        pub next: Option<Box<Node>>,
    }

    impl Node {
        pub fn new(data: i32) -> Node {
            Node { data, next: None }
        }
    }

    #[derive(Debug)]
    pub struct LinkedList {
        pub head: Option<Box<Node>>,
    }

    impl LinkedList {
        pub fn new() -> LinkedList {
            LinkedList { head: None }
        }

        pub fn push(&mut self, data: i32) {
            let new_node = Box::new(Node::new(data));
            if let Some(old_head) = self.head.take() {
                self.head = Some(Box::new(Node {
                    data: new_node.data,
                    next: Some(old_head),
                }));
            } else {
                self.head = Some(new_node);
            }
        }

        pub fn append(&mut self, data: i32) {
            let new_node = Box::new(Node::new(data));
            let mut current = &mut self.head;

            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }

        pub fn remove(&mut self, data: i32) -> bool {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if let Some(ref mut next_node) = node.next {
                    if next_node.data == data {
                        node.next = next_node.next.take();
                        return true;
                    }
                } else {
                    if node.data == data {
                        self.head = node.next.take();
                        return true;
                    }
                }
                current = &mut node.next;
            }
            false
        }

        pub fn pop(&mut self) -> bool {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if let Some(ref mut next_node) = node.next {
                    if next_node.next.is_none() {
                        node.next = next_node.next.take();
                        return true;
                    }
                }
                current = &mut node.next;
            }
            false
        }

        pub fn print(&self) {
            let mut current = &self.head;
            while let Some(node) = current {
                print!("{} -> ", node.data);
                current = &node.next;
            }
            println!("None");
        }
    }
}
