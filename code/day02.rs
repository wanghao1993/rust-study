// fn main() {
//     let mut x: u8 = 12;

//     x = 254;
//     println!("x = {}", x);
//     another_fn(x);

//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let _heart_eyed_cat = '😻';

//     println!("c={}, z={}, _heart_eyed_cat={}", c, z, _heart_eyed_cat)
// }

// fn another_fn(x: u8) {
//     println!("another fn x is {x}")
// }

// fn main() {
//     let mut x = 5.2;

//     x = x + 1.0;

//     println!("The value of x is: {x}");
// }

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// if else 控制流

// 换算华氏公式 华氏度= 32°F+ 摄氏度× 1.8.
// 换算摄氏公式 摄氏度= (华氏度- 32°F) ÷ 1.8.

// 生成第 n 个斐波那契数 fn = fn(n-1) + fn(n-2)
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("s1 is {s2}");
// }

fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    // println!("{} is s", s);

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// fn fb(n: i32) -> i32 {
//     return if n <= 1 { n } else { fb(n - 1) + fb(n - 2) };
// }
