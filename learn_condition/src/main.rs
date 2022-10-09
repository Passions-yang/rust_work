fn main() {
    let number = 3;
   // if number { // 在rust语言中，条件表达式必须是boot类型，不能是其他类型，所以此处编译有错误
     if number < 5{
        println!(" number : {}",number);
    }


    let a = 3;
    let number = if a > 0 { 1 } else { -1 };  // 可以看出，在语句块中，都没有以分号结尾，这叫表达式，表达式是有返回值的
    // 如果使用分号结尾，那么就叫语句，语句是没有返回值的。
    println!("number 为 {}", number);

    println!("Hello, world!");
}
