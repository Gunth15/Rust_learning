enum node {
    empty,
    node(i32,node),
}

struct queue {
    head: node,
    size: u32,
}
impl queue {
    fn init() {
        queue {
            head: empty,
            size: 0,
        }
    }  
    fn queue(&self, add:i32) {
       match self.head {
        empty => self.head = node(add,empty),
        node(i32,_) => { while } 
       }
       self.size++
    }

    fn dequeue(&self) -> i32 {

    }
    fn front(&self) -> i32 {

    }
}
struct stack {
    top: node,
    size: u32,
}
impl stack {


}
enum tnode {
    empty,
    node(i32,node,node),
}
struct tree {
    root: tnode
}
impl tree {

}
struct llist {
    head: node,
    size: u32,
}
impl llist {


}

fn main() {

}
