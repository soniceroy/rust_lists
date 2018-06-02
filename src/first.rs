/*
	A list is either Empty or an Element followed by a List:
	 List a = Empty | Elem a (List a)

	Type of a is "sum type": having different values which may
		be different type. AKA heterogenous type, Or Enum.
**/
use std::mem;

// pub means people outside this module can use List
pub struct List {
	head: Link,
}

impl List {
	pub fn new() -> Self {
		List { head: Link::Empty }
	}

	pub fn push(&mut self, elem: i32) {
		let new_node = Node {
			elem,
			/*
				mem::replace is an Indiana Jones maneuver.
					swaps old value with a different value,
					and returns the old value.
			*/
			next: mem::replace(&mut self.head, Link::Empty),
		};

		self.head = Link::More(Box::new(new_node));
	}

	pub fn pop(&mut self) -> Option<i32> {
		match mem::replace(&mut self.head, Link::Empty) {
			Link::Empty => None,
			Link::More(boxed_node) => {
				let node = *boxed_node; //pulls node onto stack
				self.head = node.next;
				Some(node.elem)
			},
		}
	}
}

impl Drop for List {
	fn drop(&mut self) {
		let mut cur_link = mem::replace(&mut self.head, Link::Empty);
		while let Link::More(mut boxed_node) = cur_link {
			cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
			// boxed_node goes out of scope and gets dropped here;
			// but its Node's next field has been set to Link::Empty
			// so no unbounded recursion occurs.
		}
	}
}

struct Node {
	elem: i32,
	next: Link,
}

enum Link {
	Empty,
	More(Box<Node>),
}

#[cfg(test)]
mod test {
	use super::List;
    #[test]
    fn basics() {
    	let mut list = List::new();
    	assert_eq!(list.pop(), None);

    	list.push(1);
    	list.push(2);
    	list.push(3);

    	assert_eq!(list.pop(), Some(3));
    	assert_eq!(list.pop(), Some(2));

    	list.push(4);
    	list.push(5);

    	assert_eq!(list.pop(), Some(5));
    	assert_eq!(list.pop(), Some(4));

    	while let Some(_) = list.pop() {}

    	assert_eq!(list.pop(), None);
    }
}