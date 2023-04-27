use std::io;


fn main() {
    let mut input = String::new(); // 새로운 문자열 호출, 
    let mut result = some_fn(input);
    io::stdin().read_line(&mut result); // 여기서의 input은 값을 차용(borrow)한 것이다.
    /* 
    여기서 생성된 문자열(String)은 변수 input이 소유한다.
    컴파일 시점에 문자열의 크기를 알지 못하기 때문에 힙에 저장된다.
    -> 스택에는 힙을 가리키는 포인터를 저장하고, 문자열과 같은 크기의 메타데이터를 추가로 저장한다. 
    cf) 해당 input이 범위를 벗어나면 힙에있는 문자열은 해제된다. 
        정확하게는 해당 함수가 종료되면 무자열에 있는 drop함수에가 컴퍼일러에 의해 호출되고 힙에 있는 문자열이 해제된다.

    */
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g", mars_weight);
    calculate_weight_on_mars(100.0);
}

fn some_fn(s: &mut String) -> String{
    return s;
}


fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

