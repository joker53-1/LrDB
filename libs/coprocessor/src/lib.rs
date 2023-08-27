pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//pub use protos::protos::tinykvpb::*;
//
//
//#[test]
//fn test1() {
//    let a = KeyRange { start: vec![], end: vec![]};
//    let b = Request {
//        tp: 1,
//        data: vec![],
//        start_ts: 1,
//        ranges: a,
//        context: todo!(),
//    };
//    println!("{:?}",a);
//    println!("{:?}", b);
//}