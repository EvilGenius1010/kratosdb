const MAXLAYERS: usize = 4;
use crate::helper::generate_u8;

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
            let levels = generate_u8().leading_zeros();

            for i in 0..levels {
                let i = i as usize;
                self.lists[i].insert(LinkedList { val, next: None });
            }

            //check if empty
            // self.lists[0].insert(LinkedList { val, next: None });

            self;
            //sort and push
        }

        Self {
            lists: [(); 4].map(|_| None),
        }
    }
    // //optimise to remove vector and max size is 4
    // fn determine_level()->u8{

    // }
}
