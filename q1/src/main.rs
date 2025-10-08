


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

    fn pop(&mut self)->Option<T>{
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

    fn push(&mut self,value: T){
        self.val.push(value)
    }
    fn peek(&self)->Option<&T>{
        if self.val.is_empty(){
            None
        }else{
            Some(self.val.first()?)
        }
    }
}








fn main() {     
    let e = vec![0,8,9];
    let mut b = queue::from(e);
    let a:queue<i32>= queue::new();
    println!("{b:?}");
    if let Some(len) =b.len(){
        println!("{len:?}");
    }
    b.pop();
    println!("{b:?}");
    b.push(50);
    println!("{b:?}");
    if let Some(peek) =b.peek(){
        println!("{peek:?}");
    }


}
