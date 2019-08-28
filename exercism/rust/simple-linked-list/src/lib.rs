mod node {
    #[derive(Clone, Debug)]
    pub enum Node<T: Clone> {
        Cons(Option<T>, Box<Node<T>>),
        Nil,
    }

    impl<T: Clone> Node<T> {
        pub fn new() -> Self {
            Node::Nil
        }

        pub fn len(&self) -> usize {
            match self {
                Node::Cons(_, next) => next.len() + 1,
                Node::Nil => 0,
            }
        }

        pub fn push(&mut self, element: T) {
            match self {
                Node::Cons(_, next) => next.push(element),
                Node::Nil => *self = Node::Cons(Some(element), Box::new(Node::Nil)),
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            match self {
                Node::Cons(v, next) => {
                    match next.as_ref() {
                        Node::Cons(_, _) => next.pop(),
                        Node::Nil => {
                            let v = v.take();
                            *self = Node::Nil;
                            v
                        },
                    }
                },
                _ => None,
            }
        }

        pub fn peek(&self) -> Option<&T> {
            match self {
                Node::Cons(v, _) => v.as_ref(),
                Node::Nil => None,
            }
        }

        pub fn rev(&self) -> Node<T> {
            let mut old = self.clone();
            let mut rev = Node::new();
            while let Some(val) = old.pop() {
                rev.push(val);
            }
            rev
        }
    }

    impl<'a, T: Clone> From<&'a [T]> for Node<T> {
        fn from(items: &[T]) -> Self {
            let mut list = Node::new();
            for i in items {
                list.push(i.to_owned());
            }
            list
        }
    }

    impl<T: Clone> Into<Vec<T>> for Node<T> {
        fn into(self) -> Vec<T> {
            let mut clone = self.clone();
            let mut vec = Vec::new();
            while let Some(val) = clone.pop() {
                vec.push(val);
            }
            vec.reverse();
            vec
        }
    }
}

pub use node::Node as SimpleLinkedList;
