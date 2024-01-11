// Tax calculator, State: winsconsin | illinois (8% tax), if winsconsin - county:  Eau Claire (+0.005) | Dunn (0.004) , other states: no tax.
// Inputs: order_amount, state, county(only if state: winsconsin)
// process: calculate tax
// output: The tax is: {}.\nThe total is {}.

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

fn calculate_tax(order_amount: f64, tax_percentage: f64) -> (f64, f64) {
    let tax = order_amount * (tax_percentage / 100.0);
    let total = order_amount + tax;
    (round_decimal(tax, 2), round_decimal(total, 2))
}

fn main() {
    println!("Hello, world!");
}
