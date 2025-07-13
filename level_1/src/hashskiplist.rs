const MAXLAYERS: usize = 4;

//enforce what types can be passed to this
pub struct LinkedList<V> {
    val: V,
    next: Option<Box<LinkedList<V>>>,
}

struct HashSkipList<T> {
    //hardcode values as 1,2,3,4?
    pub lists: [Option<LinkedList<T>>; MAXLAYERS],
}

impl<T> HashSkipList<T> {
    pub fn new() -> Self {
        Self {
            lists: [(); 4].map(|_| None),
        }
    }

    pub fn insert(&mut self, val: T) -> Self {
        if self.lists[0].is_some() {
            //check if empty
            self.lists[0]
            //sort and push
        }

        Self {
            lists: [(); 4].map(|_| None),
        }
    }
}
