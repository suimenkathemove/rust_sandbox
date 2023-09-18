#[derive(Debug)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

impl<T> List<T> {
    fn new() -> Self {
        Self::Nil
    }

    fn cons(self, data: T) -> Self {
        Self::Node {
            data,
            next: Box::new(self),
        }
    }

    fn iter(&self) -> ListIter<T> {
        ListIter { element: self }
    }
}

struct ListIter<'a, T> {
    element: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.element {
            List::Node { data, next } => {
                self.element = next;
                Some(data)
            }
            List::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_iter() {
        let list = List::new().cons(0).cons(1).cons(2);
        let mut list_iter = list.iter();

        assert_eq!(&2, list_iter.next().unwrap());
        assert_eq!(&1, list_iter.next().unwrap());
        assert_eq!(&0, list_iter.next().unwrap());
    }
}
