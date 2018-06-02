/*
	A list is either Empty or an Element followed by a List:
	 List a = Empty | Elem a (List a)

	Type of a is "sum type": having different values which may
		be different type. AKA heterogenous type, Or Enum.
**/
use std::mem;


// Type alias!!!
type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
	elem: T,
	next: Link<T>,
}


// pub means people outside this module can use List
pub struct List<T> {
	head: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None }
	}

	pub fn push(&mut self, elem: T) {
		let new_node = Node {
			elem,
			/*
				Option::take is implementation of mem::replace
			*/
			next: self.head.take(),
		};

		self.head = Some(Box::new(new_node));
	}

	pub fn pop(&mut self) -> Option<T> {
		// Option::map replaces the pattern
		//   node = match option { None => None, Some(x) => Some(y) }
		self.head.take().map(|node| {
			let node = *node;
			self.head = node.next;
			node.elem
		})
	}

	pub fn peek(&mut self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| {
			&mut node.elem
		})
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut cur_link = mem::replace(&mut self.head, None);
		while let Some(mut boxed_node) = cur_link {
			cur_link = mem::replace(&mut boxed_node.next, None);
			// boxed_node goes out of scope and gets dropped here;
			// but its Node's next field has been set to None
			// so no unbounded recursion occurs.
		}
	}
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		// access fields of a tuple struct numerically
		self.0.pop()
	}
}

// implementing Iter from scratch directly for List
pub struct Iter<'a, T: 'a> {
	next: Option<&'a Node<T>>,
}

impl<T> List<T> {
	pub fn iter(& self) -> Iter<T> {
		Iter {next: self.head.as_ref().map(|node| &**node)}
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

// implementing Iter_Mut from scratch directly for List
pub struct IterMut<'a, T: 'a> {
	next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut {next: self.head.as_mut().map(|node| &mut **node)}
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}
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
    }

    #[test]
    fn peek() {
    	let mut list = List::new();
    	list.push(1);
    	assert_eq!(list.peek(), Some(&1));
    	assert_eq!(list.peek_mut(), Some(&mut 1));
    }

    #[test]
    fn into_iter() {
    	let mut list = List::new();
    	list.push(1); list.push(2); list.push(3);

    	let mut iter = list.into_iter();
    	assert_eq!(iter.next(), Some(3));
    	assert_eq!(iter.next(), Some(2));

    	for _ in iter {}
    }

    #[test]
    fn iter() {
    	let mut list = List::new();
    	list.push(1); list.push(2); list.push(3);

    	let mut iter = list.iter();
    	assert_eq!(iter.next(), Some(&3));
    	for _ in iter {}
    }

    #[test]
    fn iter_mut() {
    	let mut list = List::new();
    	list.push(1); list.push(2); list.push(3);

    	let mut iter = list.iter_mut();
    	assert_eq!(iter.next(), Some(&mut 3));
    	for _ in iter {}
    }
}