
trait T1{
    fn t1_1(&self){
        println!("t1_Trait");
    }
}
trait T2{
    fn t2_1(&self){
        println!("t2_Trait");
    }
}
struct Imp {}
impl T1 for Imp{}
impl T2 for Imp{}
#[test]
fn trait_test(){
    let i=Imp{};
    i.t1_1();
    i.t2_1();
}