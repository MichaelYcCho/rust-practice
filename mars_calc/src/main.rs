use std::io;

fn main() {
    println!("Please enter your weight (kg): ");
    let mut input = String::new(); // 새로운 문자열 호출,
    io::stdin().read_line(&mut input).unwrap(); // unwrap()은 오류가 있을때 프로그램을 확실하게 종료시키는 역할이다
                                                /*
                                                여기서 생성된 문자열(String)은 변수 input이 소유한다.
                                                컴파일 시점에 문자열의 크기를 알지 못하기 때문에 힙에 저장된다.
                                                -> 스택에는 힙을 가리키는 포인터를 저장하고, 문자열과 같은 크기의 메타데이터를 추가로 저장한다.
                                                cf) 해당 input이 범위를 벗어나면 힙에있는 문자열은 해제된다.
                                                    정확하게는 해당 함수가 종료되면 무자열에 있는 drop함수에가 컴퍼일러에 의해 호출되고 힙에 있는 문자열이 해제된다.

                                                */
    let weight: f32 = input.trim().parse().unwrap(); // trim()은 문자열의 앞뒤 공백을 제거한다, parse()를 통해 문자열을 숫자로 변환한다.
    dbg!("디버거: ", weight); // dbg!는 디버깅을 위한 매크로이다.
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
