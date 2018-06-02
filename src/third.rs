/*
  Shared Ownership with a persistent Singly-Linked Stack

  	Most important feature is that manipulation of the tails
  	is basically free:
  		list1 = A -> B -> C -> D
  		list2 = tail(list1) = B -> C -> D
  		list3 = push(list2, X) = X -> B -> C -> D
  		# with memory looking like this:
  			list1 -> A ---v
  			list2 ------> B -> C -> D
  			list3 -> X ---^
  	Boxes can't work for this because the ownership is shared
  		so the compiler can't tell who should free it.

  	Rc (reference counting) can be used instead
  	-- cost is that they have to be immutable
*/
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

pub struct List<T> {
	head: Link<T>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List {head: None}
	}

	pub fn append(&self, elem: T) -> List<T> {
		List {head: Some(Rc::new(Node {
			elem,
			next: self.head.clone(), // clone is exposed by Option
									 // Rc uses clone to increase
									 // the reference count
		}))}
	}

	pub fn tail(&self) -> List<T> {
		// and_then "flat maps" to an Option<T>
		// since self.next is an Option<T>, 
		//   using map here would nest Option<Option<T>>
		List {head: self.head.as_ref().and_then(|node| node.next.clone())}

	}

	pub fn head(&self) -> Option<&T> {
		self.head.as_ref().map(|node| &node.elem)
	}

}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn basics() {
		let list = List::new();
		assert_eq!(list.head(), None);

		let list = list.append(1).append(2).append(3);
		assert_eq!(list.head(), Some(&3));

		let list = list.tail();
		assert_eq!(list.head(), Some(&2));

		let list = list.tail();
		assert_eq!(list.head(), Some(&1));

		let list = list.tail();
		assert_eq!(list.head(), None);

		// Make sure empty tail works
		let list = list.tail();
		assert_eq!(list.head(), None);
	}
}