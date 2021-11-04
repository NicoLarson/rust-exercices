fn main() {
    let prices = [1,1,7,3,6,4];

    let mut sort_prices = prices;
    sort_prices.sort();

    let the_bigger_price = sort_prices[sort_prices.len() - 1];
    let the_lower_price = sort_prices[0];
    let the_bigger_price_after_the_lower_price = the_biger_price_after_the_lower_price(prices, the_lower_price);

    let mut big_price = false;
    let mut lower_price = false;
    let mut bigger_after_lower = false;

    println!("{:?}",prices);

    let mut i: usize = 0;
    while i < prices.len(){
        if the_lower_price == prices[i]{
            lower_price = true;
        }
        if the_bigger_price == prices[i]{
            big_price = true;
        }
        if the_bigger_price_after_the_lower_price == prices[i] && lower_price == true && big_price == true{
            println!("The best day to buy is {}, the best day to sell is {}",the_day(prices, the_lower_price),the_day(prices, the_bigger_price_after_the_lower_price));
        }
        if lower_price == true && big_price == false{
            println!("The best day to buy is {}, the best day to sell is {}",the_day(prices, the_lower_price),the_day(prices, the_bigger_price));
            break;
        }
        i += 1;
    }
}

fn the_day(table: [i32;6], the_price: i32)->usize{
    let mut i: usize = 0;
    let mut result = 0;
    while i < table.len(){
        if table[i] == the_price{
            result = i;
        }
        i+=1;
    }
    result
}

fn the_biger_price_after_the_lower_price(table: [i32;6], the_lower_price: i32) -> i32{
    let mut i: usize = 0;
    let mut result = 0;
    let mut  after_the_lower_price = false;
    while i < table.len(){
        if the_lower_price == table[i]{
            after_the_lower_price = true;
        }
        if after_the_lower_price == true{
            if result < table[i]{
                result = table[i];
            }
        }
        i += 1;
    }
    result
}

#[test]
