use std::cmp::PartialOrd;
use std::cmp::Ordering;


#[derive(Debug)]
enum BTree<T: PartialOrd> {
    Empty,
    Assemble(T, Box<BTree<T>>, Box<BTree<T>>)
}



fn insert<T: PartialOrd>(data: T, b: BTree<T>) -> BTree<T> {
    use BTree::*;
    match b {
        Empty => Assemble(data, Box::new(Empty), Box::new(Empty)),
        Assemble(root, left, right) => {
            if data.partial_cmp(&root) == Some(Ordering::Less) {
                println!("left");
                Assemble(root, Box::new(insert(data, *left)), right)
            } else {
                println!("right");  
                Assemble(root, left, Box::new(insert(data, *right)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::BTree;
    use crate::BTree::*;
    use crate::insert;

    #[test]
    fn test_ctor_empty() {
        let a = BTree::<i32>::Empty;
        println!("{:#?}", a);
    }

    #[test]
    fn test_data_access() {
        let left = Box::new(Empty);
        let right = Box::new(Empty);
        let b = Assemble(200, left, right);
        match b {
            Empty => println!("nothin'"),
            Assemble(data, left, right) => println!("data: {}", data)
        }
    }

    #[test]
    fn test_insert_one() {
        let b = insert(43, Empty);
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_2() {
        let mut b = Empty;
        let mut i = 0;
        while i < 2 {
            b = insert(i, b);
            i += 1;
        }
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_3() {
        let mut b = Empty;
        let mut i = 0;
        while i < 3 {
            b = insert(i, b);
            println!("{:#?}", b);
            i += 1;
        }
    }

    #[test]
    fn test_insert_7() {
        let mut b = Empty;
        let mut i = 0;
        while i < 7 {
            b = insert(i, b);
            i += 1;
        }
        println!("{:#?}", b);
    }

}

