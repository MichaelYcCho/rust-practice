pub fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32{
    let d = 5; // 지역변수 생성 stack에 할당
    let e = Box::new(7); // heap에 할당하는 방법중하나
    return d + *e;
}