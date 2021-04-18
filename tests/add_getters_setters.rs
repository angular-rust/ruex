// #[macro_use]
// extern crate add_getters_setters;

// #[derive(AddGetter, AddGetterVal, AddGetterMut, AddSetter)]
// struct Ts {
//     jaf: u8,

//     #[set]
//     #[get_val]
//     field_1: u8,

//     #[get]
//     #[get_mut]
//     field_2: String,
// }

// // these functions shouldn't be set since there are not attrs on jaf. if they are set then it wont compile because these would be duplicate function definitions, so then we'd know theres something wrong.
// impl Ts {
//     #[allow(dead_code)]
//     pub fn get_jaf(&self) -> & u8 {
//         &self.field_1
//     }

//     #[allow(dead_code)]
//     pub fn jaf(&self) -> u8 {
//         self.jaf
//     }

//     #[allow(dead_code)]
//     pub fn get_jaf_mut(&mut self) -> &mut u8 {
//         &mut self.field_1
//     }

//     #[allow(dead_code)]
//     pub fn set_jaf(&mut self, v: u8) {
//         self.jaf = v;
//     }
// }

// #[test]
// fn test_add_setter() {
//     let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     a.set_field_1(14);
//     assert_eq!(a.field_1, 14);
// }

// #[test]
// #[should_panic]
// fn test_add_setter_should_panic() {
//     let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     a.set_field_1(20);
//     assert_eq!(a.field_1, 11);
// }

// #[test]
// fn test_add_getter() {
//     let a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     assert_eq!(a.get_field_2(), &String::from("hello"));
// }

// #[test]
// fn test_add_getter_mut() {
//     let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     let b = a.get_field_2_mut();
//     *b = String::from("world");
//     assert_eq!(a.get_field_2(), &String::from("world"));
// }

// #[test]
// #[should_panic]
// fn test_add_getter_mut_should_panic() {
//     let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     let b = a.get_field_2_mut();
//     *b = String::from("world");
//     assert_eq!(a.get_field_2(), &String::from("hello"));
// }

// #[test]
// fn test_add_getter_by_val() {
//     let a = Ts {jaf: 4, field_1: 5, field_2: String::from("hello")};
//     let b = a.field_1();
//     assert_eq!(b, 5);
// }

// #[test]
// #[should_panic]
// fn test_add_getter_by_val_should_panic() {
//     let a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
//     let b = a.field_1();
//     assert_eq!(b, 5);
// }

// // *********************************
// // * test tags on the whole struct *
// // *********************************

// #[derive(Debug, PartialEq)]
// enum DragonClassifications {
//     BlackDragon,
//     LuckDragon,
// }

// #[derive(AddGetter, AddGetterMut, AddSetter)]
// #[get]
// #[get_mut]
// #[set]
// struct Dragon {
//     name: String,
//     age: u64, // 18446744073709551615 year old dragons cos why not
//     ty: DragonClassifications
// }

// #[test]
// fn get_dragon_name() {
//     let smaug = Dragon {
//         name: "Smaug".to_owned(),
//         age: 171,
//         ty: DragonClassifications::BlackDragon
//     };
//     assert_eq!(*smaug.get_name(), "Smaug".to_owned());
// }

// #[test]
// fn get_dragon_age_mut() {
//     let mut smaug = Dragon {
//         name: "Smaug".to_owned(),
//         age: 171,
//         ty: DragonClassifications::BlackDragon
//     };
//     *smaug.get_age_mut() = 172;
//     assert_eq!(*smaug.get_age(), 172);
// }

// #[test]
// fn set_dragon_type() {
//     let mut falkor = Dragon {
//         name: "Falkor".to_owned(),
//         age: 0xffffffffffffffff,
//         ty: DragonClassifications::BlackDragon
//     };
//     falkor.set_ty(DragonClassifications::LuckDragon);
//     assert_eq!(*falkor.get_ty(), DragonClassifications::LuckDragon);
// }

// // ***************************
// // * if statement benchmarks *
// // ***************************

// // uncomment them and paste them into your own file to try for yourself
// // my results:
// //  bench_try_func_call_first ....... 7 ns/iter (+/- 0)
// //  bench_try_func_call_last ........ 0 ns/iter (+/- 0)

// // #[cfg(test)]
// // mod benches {
// //     use test::Bencher;

// //     fn all_in_range<'a, T: Iterator<Item = &'a u64>>(mut attribs: T, low: u64, high: u64) -> bool {
// //         attribs.find_map(|v| {
// //             if *v <= high && *v >= low {
// //                 Some(v)
// //             } else {
// //                 None
// //             }
// //         }).is_some()
// //     }

// //     #[bench]
// //     fn bench_try_func_call_first(b: &mut Bencher) { // 7 ns/iter
// //         let can_pass = true;
// //         let collection: Vec<u64> = vec![34, 5431, 12344, 734125, 65426, 276, 7, 6, 7487, 987, 569, 222, 333, 444, 555, 666, 777, 888, 984302, 12, 1, 2, 3, 4, 18];
// //         b.iter(|| {
// //             test::black_box(
// //                 if all_in_range(collection.iter(), 800, 900) || can_pass {
// //                     18u8 // just return some random stuff so the complier doesn't skip code, see https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html#gotcha-optimizations
// //                 } else {
// //                     5u8 // just return some random stuff so the complier doesn't skip code, see https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html#gotcha-optimizations
// //                 }
// //             )
// //         });
// //     }

// //     #[bench]
// //     fn bench_try_func_call_last(b: &mut Bencher) { // 0 ns/iter
// //         let can_pass = true;
// //         let collection: Vec<u64> = vec![34, 5431, 12344, 734125, 65426, 276, 7, 6, 7487, 987, 569, 222, 333, 444, 555, 666, 777, 888, 984302, 12, 1, 2, 3, 4, 18];
// //         b.iter(|| {
// //             test::black_box(
// //                 if can_pass || all_in_range(collection.iter(), 800, 900) {
// //                     18u8 // just return some random stuff so the complier doesn't skip code, see https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html#gotcha-optimizations
// //                 } else {
// //                     5u8 // just return some random stuff so the complier doesn't skip code, see https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html#gotcha-optimizations
// //                 }
// //             )
// //         });
// //     }
// // }
