/// 堆分配
fn main() {
    let stack_data = 32; // 栈
    let heap_data = Box::new(32); // 堆

    let box1 = Box::new(10);
    let raw_ptr = Box::into_raw(box1); // 转成裸指针
    let back_to_box = unsafe { Box::from_raw(raw_ptr) }; // 裸指针转回Box
}
