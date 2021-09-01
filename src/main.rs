use std::collections::{HashMap, VecDeque};

pub trait ValueHelper {
    fn touch_last(&self) -> &str;
}

impl ValueHelper for VecDeque<&str> {
    fn touch_last(&self) -> &str {
        &self[self.len()-1]
    }
}

fn convert2post(input: String) -> String {
    let mut post_map = HashMap::new();
    post_map.insert("not", 3);
    post_map.insert("and", 2);
    post_map.insert("or", 1);

    let mut st:VecDeque<&str> = VecDeque::new();
    let mut post = VecDeque::new();
    let str_list = input.split_whitespace();
    for i in str_list{
        if !post_map.contains_key(i){
            post.push_back(i);
        }else{
            let p1 = *post_map.get(i).unwrap();
            let mut p2 = 0;
            if st.len() == 0{

            }else{
                p2 = *post_map.get(st.touch_last()).unwrap();
            }
            if i != ")" && ( st.len() == 0 || i == "(" || st.touch_last() == "(" || p1 > p2  ){
                st.push_back(i)
            }else if  i == ")" {
                loop {
                    let tmp = st.pop_back();
                    match tmp{
                        None => {break}
                        Some(v) => {
                            if v != "("{
                                post.push_back(v);
                            }else{
                                break;
                            }
                        }
                    }
                }
            }else{
                loop{
                    if st.len() > 0 && st.touch_last() != "(" {
                        let p1 = *post_map.get(i).unwrap();
                        let p2 = *post_map.get(st.touch_last()).unwrap();
                        if p1 <= p2{
                            post.push_back(st.pop_back().unwrap());
                        }else{
                            st.push_back(i);
                            break;
                        }

                    }else{
                        st.push_back(i);
                        break;
                    }
                 }
            }
        }
    }
    while st.len() > 0{
        post.push_back( st.pop_back().unwrap());
    }

    let mut res = "".to_string();
    for i in post{
        res.push_str(i);
        res.push_str(" ");
    }
    res

}

fn main() {

    let a ="A and B or ( not C and D  )".to_string();
    let res = convert2post(a);
    println!("{}", res);



}