fn main() {
    let nums: Vec<i32> = vec![-1,1,0,-3,3];
    println!("{:?}",nums);
    println!("{:?}",product_except_self(nums));
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

    let mut result = vec![];
    let mut prod = 1;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < nums.len(){
        while j < nums.len(){
            if j != i{
                prod *= nums[j];
            }
            j+=1;
        }
        println!("{}",prod);
        result.push(prod);
        prod=1;
j=0;
        i+=1;
    }
    result
}
