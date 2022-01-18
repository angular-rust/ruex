#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ruex_macro::*;

// #[Aspect {
//     advice: "aspects::TestAspect",
//     before: "before(val,val2)", 
//     after: "after()"
// }]
// fn test_aop(&self, val: i32) -> Result<String, AppErr> {
//     let joint_point = move || {
//         println!("Closure {:?} with {}", self, val)
//     };

//     joint_point();

//     Ok(String::from("Good"))
// }

#[derive(Debug)]
enum AppErr {
    WrongParam,
}

mod aspects {
    use super::AppErr;

    struct TestAspect;

    impl TestAspect {
        fn before(val: i32) -> Result<(), AppErr> {
            println!("called before");
            Ok(())
        }

        fn after(val: Result<String, AppErr>) -> Result<String, AppErr> {
            println!("called after");
            Ok(String::from("Here"))
        }
    }
}

#[derive(Debug)]
struct AopExample;

impl AopExample {
    #[Aspect {
        advice: "aspects::TestAspect",
        before: "before(val)", 
        after: "after()"
    }]
    fn test_aop(&self, val: i32) -> Result<String, AppErr> {
        Ok(String::from("Good"))
    }
}

fn main() {
    let ex = AopExample;
    println!("HERE {:?}", ex.test_aop(10));
}