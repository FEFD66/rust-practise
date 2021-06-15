#[test]
pub fn lifetime_mark() {
    let s1 = String::from("111111");
    let s2 = String::from("22222");
    let ss ;
    {
        ss=longest(&s1,&s2);
    }
    println!("{}",ss);
}
/*
省略lifetime 标注的三个条件：
第一条规则是每一个是引用的参数都有它自己的生命周期参数。
第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)
 */
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
