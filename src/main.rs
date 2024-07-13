struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            data,
            next: None,
        }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new () -> LinkedList {
        LinkedList { head: None }
    }

    fn push(&mut self, data: i32) {
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

    fn append(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }

    fn bottom(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node. data
        })
    }

    fn top(&mut self) -> Option<i32> {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                return Some(node.data);
            }
            current = &mut node.next;
        }
        None
    }

    fn remove(&mut self, data: i32) -> bool {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.data == data {
                    node.next = next_node.next.take();
                    return true;
                } else {
                    if node.data == data {
                        self.head = node.next.take();
                        return true;
                    }
                }
            }
            current = &mut node.next;
        }
        false
    }

    fn pop(&mut self) -> bool {
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

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push(1);
    ll.push(2);
    ll.append(3);
    ll.append(4);

    ll.print();

    if let Some(value) = ll.bottom() {
        println!("Popped value: {}", value);
        ll.remove(value);
    } else {
        println!("List is empty");
    }
    ll.print();

    if let Some(value) = ll.top() {
        println!("Top value: {}", value);
        ll.pop();
    } else {
        println!("List is empty");
    }
    ll.print();
}