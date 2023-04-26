mod first;  // first.rs 파일을 모듈로 정의

fn main() {
    let a = 2;
    let result = first::stack_only(a);
    dbg!(result);
}
