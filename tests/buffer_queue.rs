use rust_gears_sdk::bcossdkutil::bufferqueue::BufferQueue;

#[test]
pub fn test_queue() {
    let mut queue = BufferQueue::new();
    let mut v: Vec<u8> = [1, 2, 3, 4, 5].to_vec();
    queue.append(&mut v);
    println!("{:?}", queue);
    let mut v1: Vec<u8> = [6, 7, 8].to_vec();
    queue.append(&mut v1);
    println!("{:?}", queue);
    queue.cut(3);
    println!("{:?}", queue);
}
