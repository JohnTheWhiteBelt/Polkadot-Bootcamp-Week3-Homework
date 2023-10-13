fn main() {
    let vec = vec![1,2,3,4,5,6,7,8,9,10];
    let condition = FilterCondition {value: 5};
    let filtered = custom_filter(vec, &condition);
    println!("The filtered result is : {:?}", filtered);
}

struct FilterCondition {
    value: i32
}

impl FilterCondition {
    fn is_match(&self, input: i32) -> bool {
        input > self.value
    }

}
fn custom_filter(coll: Vec<i32>, con: &FilterCondition ) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for item in coll{
        if con.is_match(item) {
            result.push(item);
        }
    }
    result
}