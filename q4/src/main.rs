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
        if self.back < 1024{
            self.val[self.back] = val; 
            self.back += 1;
        }else{
            self.back = (self.back+1) % 1024;
        }
    }
    fn removed(&mut self){
        if self.front <= 1024{
            self.val[self.front] = 0;
            self.front += 1;

        }else{
            self.front = (self.front+1) % 1024;
        }
    }
}








fn main() {
    let mut a = circle::new();
    println!("{a:?}");
    for i in 0..1025{

        a.add(5);
    }
    println!("{a:?}");
    for i in 0..1024{
        a.removed();
    }
    println!("{a:?}");
}
