/*A library for calculate how many Chicken and Rabbit in the Same Cage when the total number of heads and legs are given */

pub fn get_chicken(thead: i32, leg: i32) -> i32 {
    let mut chicken: i32 = -1;
    for i in 0..thead {
        let y: i32 = thead - i;

        if 2 * i + 4 * y == leg {
            chicken = i;
            break;
        }
    }
    chicken
}
pub fn get_rabbit(chicken: i32, thead: i32) -> i32 {
    thead - chicken
}
