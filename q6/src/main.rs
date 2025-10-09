
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
            Some(self.str.last()?)
        }
    }
    fn len(&self)-> usize{
        self.str.len()
    }
    fn is_empty(&self)-> bool{
        self.str.is_empty()
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

//from a string :( very difficult :((((((((((((

impl<T : From<char>> From<String> for Stack<T>{
    fn from(value: String) -> Self {
        Self{
            str: value.chars().map(T::from).collect(),
        }
    }
}

impl Stack<char>{
    fn dup(&mut self){
        let mut a: Stack<char> = Stack::new();
        while let Some(item) = self.pop(){
            if let Some(&top) = a.peek(){
                if top == item{
                    a.pop();
                    continue;
                }
            }
            a.push(item)
        }
        while let Some(c) = a.pop(){
            self.push(c);
        }

    }
    fn q6(&mut self)-> bool{
        let mut a:Stack<char> = Stack::new();
        while let Some(item) = self.pop(){
            match item {
                ')' | '}' | ']' => a.push(item),
                '(' => {
                    if a.pop() != Some(')'){
                        return false;
                    }
                },
                '{' => {
                    if a.pop() != Some('}'){
                        return false;
                    }
                },
                '[' => {
                    if a.pop() != Some(']'){
                        return false;
                    }
                },
                _ => continue

            }

        }
        a.is_empty()
    }


}













fn main() {
    let mut a: Stack<char> = Stack::from(String::from("[a{b]c}"));
    let b = a.q6();
    println!("{b:?}");
    println!("{a:?}");

}
