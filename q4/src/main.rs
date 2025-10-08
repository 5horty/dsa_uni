#[derive(Debug)]
struct circle{
    val : [i32;1024],
    front : usize,
    back : usize,
}
impl circle{
    fn new()-> Self{
        Self{
            val: [0;1024],
            front: 0,
            back: 0,

        }
    }
    fn add(&mut self, val: i32){
        let next = (self.back+1) % 1024;
        if next == self.front{
            return;
        }
        self.val[self.back] = val; 
        self.back = next;
    }
    fn removed(&mut self){
        if self.front == self.back{
            return;
        }
        self.val[self.front] = 0;
        self.front = (self.front+1) % 1024;
    }
    fn is_empty(&self)->bool{
        self.front == self.back
    }
    fn is_full(&self)-> bool{
        (self.back + 1)% 1024 == self.front  
    }
}








fn main() {
    let mut a = circle::new();
    for i in 0..=1025{

        a.add(5);
    }
    println!("{}",a.is_full()); // true
    println!("{a:?}");
    for i in 0..=1024{
        a.removed();
    }
    println!("{}",a.is_empty()); // true
    println!("{a:?}");
}
