pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter{
            next: self.head.as_deref()
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a,T>{
    next: Option<&'a Node<T>>,
}

/// KEYPOINT:
/// 从Option<Box<T>>到Option<&T>的转换
/// node.next 的类型是Option<Box<Node<T>>>
/// 1. as_deref()
/// 会转换Option<T> (or &Option<T>) to Option<&T::Target>.
/// 由于智能指针的实现了Deref trait 所以相当于*Box<T> => &T
/// 2. as_ref().map::<&Node<T>, _>(|node| node)
/// as_ref()将&Option<Box<Node<T>>>转换为Option<&Box<Node<T>>>
/// map fn接受一个闭包，闭包的参数是&Box<Node<T>>
/// Option map的函数签名是
/// ```pub fn map<U, F>(self, f: F) -> Option<U>
/// where
///     F: FnOnce(T) -> U,
/// ```
/// 用涡轮鱼语法指定map的返回类型(泛型参数)为&Node<T>
/// 交给编译器推断node的转换过程
/// 3. as_ref().map(|node| &**node)
/// as_ref()将&Option<Box<Node<T>>>转换为Option<&Box<Node<T>>>
/// map fn接受一个闭包，闭包的参数是&Box<Node<T>>
/// 手动处理node的转换过程
/// 第一次解引用 &Box<Node<T>> => Box<Node<T>>
/// 第二次解引用 Box<Node<T>> => Node<T>
/// 返回引用的值 &Node<T>
impl<'a,T> Iterator for Iter<'a,T>  {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| node);
            // self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
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

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }


}
