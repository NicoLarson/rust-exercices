fn main() {
    let nums = [1,2,1,4];

    let mut i = 0;
    let mut j = 0;

    let mut is_duplicate = false;
    println!("{:?}",nums);

    while i<nums.len(){
        while j<nums.len(){
            if j != i && nums[i] == nums[j]{
                is_duplicate =true;
            }
            j+=1;
        }
        i+=1;
    }
    println!("{}",is_duplicate);
}
