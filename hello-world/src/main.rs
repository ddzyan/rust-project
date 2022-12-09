fn main() {
    let s1 = String::from("hello");
    let mut s2 = &s1; // 向 s1 租借所有权
    let s3 = s1; // s1 所有权转移到 s3
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);
}
