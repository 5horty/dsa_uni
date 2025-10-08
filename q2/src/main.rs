#[derive(Debug)]
struct Stack<T>{
    str : Vec<T>
}

impl<T> Stack<T>{
    fn new()-> Self{
        Self{
            str: Vec::new()
        }
    }
    fn pop(&mut self)->Option<T>{
        if self.str.is_empty(){
            None
        }else{
            Some(self.str.pop()?)
        }
    }
    fn push(&mut self,item: T){
        self.str.push(item);
    }
    fn peek(&self)->Option<&T>{
        if self.str.is_empty(){
            None
        }else{
            Some(self.str.first()?)
        }
    }
    fn len(&self)-> usize{
        self.str.len()
    }
}
// creating the stack from an vector :)
impl<T> From<Vec<T>> for Stack<T>{
    fn from(value: Vec<T>) -> Self {
        Self{
            str: value
        }
    }
}



#[derive(Debug)]
enum test{
    int(i32),
    string(String),
}



fn main() {
    let t = vec![0,8,6,7];
    let mut b = Stack::from(t);
    let a: Stack<i32> = Stack::new();
    println!("{a:?}");
    println!("{b:?}");
    b.pop();
    println!("{b:?}");
    b.push(50);
    println!("{b:?}");
    if let Some(f) = b.peek(){
        println!("{f:?}");
    }
    let r = b.len();
    println!("{r:?}");
    
}
