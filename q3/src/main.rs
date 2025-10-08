
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
struct queue<T>{
    val: Vec<T>,
}
impl<T> From<Vec<T>> for queue<T>{
    fn from(value: Vec<T>) -> Self {
       Self{
            val: value,
        } 
    }
}
impl <T>queue <T>{
  fn new() -> Self{
        Self{
            val: Vec::new(),
        }

    }  

    fn dequeue(&mut self)->Option<T>{
        if self.val.is_empty(){
            None
        }else{
            Some(self.val.remove(0))
        } 

        
    }
    fn len(&self)->Option<usize>{
        if self.val.is_empty(){
            None
        }else{
            Some(self.val.len())

            
        }
    }

    fn enqueue(&mut self,value: T){
        self.val.push(value)
    }
    fn peek(&self)->Option<&T>{
        if self.val.is_empty(){
            None
        }else{
            Some(self.val.first()?)
        }
    }
    fn reverse(&mut self) {
        let mut a = Stack::new();
            while let Some(item) = self.dequeue(){
                a.push(item);
        }
           while let Some(item) = a.pop(){
                self.enqueue(item);
        }
    }
}





fn main() {
    let a = vec![10,20,30,40];
    let mut q = queue::from(a);
    println!("{q:?}");
    q.reverse();
    println!("{q:?}");


}
