use std::collections::HashMap;

// Request 구조체의 수명 buf와 같은 수명을 가진다.
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

// 힙 할당 동적 Array = 백터(Vec) -> 정해지지 않은 수의 Element를 가질 수 있다.
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    // 문자열 슬라이스를 파라미터로 받는다
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i]; // = 앞의 문자열
                val = &sub_str[i + 1..]; // = 뒤의 문자열
            }

            data.entry(key)
                // Key가 이미 존재하는 경우 체크(entry)
                //  F: FnOnce(&mut V) -> 익명함수를 타입화 한것
                // existing -> 기존의 값을 받아옴
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        //방법 1
                        /*
                        let mut vec = Vec::new();
                        vec.push(val);
                        vec.push(prev_val);
                         */
                        // *existing => 참조 해제시 사용
                        // 이전에 Single이었던 값이 Multiple로 변경된다.
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                // Key가 존재하지 않는 경우
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
