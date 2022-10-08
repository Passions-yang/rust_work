
const MAX_POSTION:u32 = 10000;

fn main() {
    let a = 5; // 不声明属性，默认一般是有符号类型
    let b:u32 = 5;
    println!("a = {}",a);
    println!("b = {}",b);
    // a = 100;  由于a 是不可更改变量，所以不能在此处更改
    println!("a = {}",a); 
    let b = 10; // 隐藏性，此处定义会立即生效，并且隐藏掉上面的不可变量b。
    println!("b = {}",b);
    let mut a = 100;  // 如果要更改变量，那么必须将变量声明为mut类型，表明是一个可变变量
    println!("a = {}",a); 
    a = 300;
    println!("a = {}",a);   

    println!("MAX_POSTION = {}",MAX_POSTION);
   // MAX_POSTION = 2000; 由于 MAX_POSTION 是一个常量，所以不可更改
    println!("MAX_POSTION = {}",MAX_POSTION);
    println!("Hello, world!");
}
