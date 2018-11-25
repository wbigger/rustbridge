
fn add(n:i32) -> i32 {
    if n> 50 {
    12+n
    }
    else {
        -12+n
    }
}
fn main() {
    let mut apples :u16 = 100;
    let delta = 50u16;
    apples += delta;    
    println!("I have {} apples",apples);
    let x = add(1);
    let _y = add(1);
    println!("{}",x);
    //panic!("Oh no!");
    match x {
        0...100 => println!("inside 100"),
        _ => {println!("outside 100");},
    }
    let color = [255,0,100];
    let index = 4;
    println!("{:?}",color[index]);

    let mut prices = vec![1,2,3];
    prices[0] = 99;
    prices.push(99);
    println!("{:?}",prices);
    
}

