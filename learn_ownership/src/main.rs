// 所有权
/**
 *  程序在运行时需要管理内存资源，所以rust提出了所有权的概念
 */
//  所有权规则
/**
 * 1. rust中的每一个值都有一个变量，称为所有者
 * 2. 一次只能有一个所有者
 * 3. 当所有者不在运行范围时，该值将删除
 */

 // 变量范围
/**
{
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围
}
// 变量范围已经结束，变量 s 无效
 * */

 // 变量与数据交互
 /**
  *  变量和数据交互的方式有两种移动（move）和克隆（clone）
  */
  // 移动
  /**
   {
        let x = 5;
        let y = x; 此程序没有分配内存，所以在栈中分配内存，
        这个程序将值 5 绑定到变量 x，然后将 x 的值复制并赋值给变量 y。现在栈中将有两个值 5。

   }
   */
// 使用移动方式进行数据交互的类型都是基本类型，基本类型有：
/**
 *  1. 所有整数类型，i32,u32,i64,u64等
 *  2. 布尔类型bool
 *  3. 所有浮点类型
 *  4. 字符类型char
 *  5. 以上基本数据类型组成的元组（Tuple）
 */
// 上述基本类型发生数据交互，全是移动，并且在栈中

// 如果是其他类型，那么交互就发生在堆中
// 例如：test_move()

fn test_move(){
    let s1 = String::from("hello"); // 由于不是基本数据类型，那么hello就在堆中存放
    let s2 = s1; // 为了防止重复释放，当s1移动给s2之后，s1变量就无效了
    println!("{}",s2);
}
// 如果解决上述方式；就需要使用克隆技术，再克隆一份内存
fn test_clone(){
    let s1 = String::from("hello"); // 由于不是基本数据类型，那么hello就在堆中存放
    let s2 = s1.clone(); // 为了防止重复释放，当s1移动给s2之后，s1变量就无效了
    println!("{},{}",s2,s1);
}

// 函数的所有权机制
fn takes_ownership(some_string: String) { // some_string 参数获得了传递变量的所有权
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn test_quan(){
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s); // 将变量传递给函数，使用的是变量之间的数据交互方式是移动
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5; 
    // x 被声明有效

    makes_copy(x); // 由于 x是基本类型，所有本身就使用的是移动交互方式，所以无需释放，再后续的调用中依然有效
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s

} // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放


/* 重点：

    当函数返回值的变量所有权将会被移出函数并返回到调用函数的地方，而不会被直接释放
*/

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 声明有效
    return some_string; // 意味着转移的是"hello"的地址
    // 当函数返回值的变量所有权将会被移出函数并返回到调用函数的地方，而不会被直接释放
}

fn takes_and_gives_back(a_string: String) -> String { 

    a_string 
}

fn test_return(){
    let s1 = gives_ownership();
    // s1获得 gives_ownership() 返回的所有权
    let s2 = String::from("hello");
    // s2 声明有效
    let s3 = takes_and_gives_back(s2); // 根据数据交互规则，所有权产生了移动，
    // 将所有权传递给了 takes_and_gives_back()形参，该函数直接返回，又将所有权交还给了s3
    //  所以综上所述，在后续过程中，s1,s3 均可以使用，s2已经丢失所有权。
    println!("s1:{}, s3:{}",s1,s3);
   // println!("s1:{}, s2: {},s3:{}",s1,s2,s3); s2 已经丢失所有权，所以编译报错
}


// 引用和租借
/**
 * 什么是引用？
 *  引用： 是变量的间接访问方式。
 *  
 */

 // 例如：
 fn test_quote(){ // 引用

    let s1 = String::from("hello");
    let s2 = &s1;   // 没有使用移动和克隆的方式，而是使用引用的方式，
    println!("s1 is {}, s2 is {}", s1, s2);
 }
 // 引用性质
 /**
  *   1. 引用不会获得值的所有权
  *   2. 引用只能租借值的所有权
  *   3. 引用本身也是一个类型，并具有一个值，这个值记录的是别的值所在的位置，但引用并不具有所有值的所有权
  *   4. 一个可变变量最多只能被引用一次（租借一次），
  */

fn test_quote2(){
    let s1 = String::from("hello"); // 声明有效
   // let s2 = &s1;   // s2 是 s1的引用，并不具有s1的所有权,而s2 是一个不可变变量
    let s3 = s1;    // s3 是s1 变量的移动，所以s3具有了s1的所有权，这样意味着s2已经无法引用到s1
    // 所以不可能编译通过，因为s3获得了所有权，而s2租借了s1的所有权，s1的所有权转移给了s3
    println!("{}", s3);
}

fn test_quote3(){
    let s1 = String::from("hello");
    let mut s2 = &s1; // 根据函数test_quote2()的分析，s2需要重新引用s3，所以s2必须是一个可变变量
    let s3 = s1;
    s2 = &s3; // s2 重新租借 s3的所有权
    println!("{}", s2);

   // s2.push_str("oob"); // 错误，禁止修改租借值，因为s3就是一个不可变值

   let mut s4 = String::from("run");
    // s1 是可变的

    let s5 = &mut s4; // s5租借的s4的权，由于s4本身是可变值，所以，可以进行修改
    // s2 是可变的引用

    s5.push_str("oob");
    println!("s5： {}", s5);

    let mut s = String::from("hello");

    let r1 = &mut s;
  //  let r2 = &mut s;  // 由于s已经被r1引用，所以就不能进行第二次引用

    println!("{}", r1);

}

//垂悬引用（ Dangling References）
fn dangle() -> &String {
    let s = String::from("hello"); // 首先s所指向的内存在堆上申请。
    //  
    &s // 返回一个引用，而引用的内存在栈上，此处没有分号，表明是一个表达式，是可以返回值的
    // 当返回s的引用后，s在栈上面的内存就丢失了，意味着接受此函数返回值的变量无法拿到堆上”hello“的地址
    // 所以没有发生所有权的转移，因此堆上内存被释放，s被释放
}
fn test_quote4(){
    let reference_to_nothing = dangle();
    // 导致 reference_to_nothing 被悬空，也就相当于c中的野指针，
    // rust编译器可以查找到此错误，在编译阶段就发现问题，因此编译失败。
}


fn main() {

    test_move();
    test_clone();
    test_quan();
    test_return();
    test_quote();
    test_quote2();
    test_quote3();
    println!("Hello, world!");
}
