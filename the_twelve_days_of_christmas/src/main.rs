fn main() {
    let days = ["first", "second", "third", "fourth"];
    let gifts = ["And a partridge in a pear tree.\n",
                "Two turtle doves",
                "Three French hens,",
                "Four calling birds,"];
    let mut days_index = 0;
    let mut gifts_index = 0;
    
    while days_index < days.len() {
        println!("On the {} day of Christmas",days[days_index]);
        println!("My true love gave to me");
        if days_index == 0 {
            println!("A partridge in a pear tree.\n");
        }
        else {
            gifts_index = days_index;
            while gifts_index >= 0 {
                println!("{}",gifts[gifts_index]);
                if gifts_index == 0 {
                    break;
                }
                else {
                    gifts_index -= 1;
                }      
            }
        }
        days_index += 1;
    }    
}
