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

mod tests {
    use super::*;

    #[test]
    fn test_calculate_tax() {
        assert_eq!(calculate_tax(10.0, 5.5), (0.55, 10.55));

        // Check for zero order amount, expecting zero tax and total
        assert_eq!(calculate_tax(0.0, 5.0), (0.0, 0.0));

        // Check for negative tax percentage, expecting negative tax and total
        assert_eq!(calculate_tax(200.0, -8.0), (-16.0, 184.0));

        // Check for large order amount and tax percentage
        assert_eq!(calculate_tax(999999.99, 20.0), (200000.0, 1199999.99));
    }

    #[test]
    fn test_round_decimal() {
        // Test rounding to 2 decimal places
        assert_eq!(round_decimal(3.14159, 2), 3.14);
        assert_eq!(round_decimal(6.666666, 2), 6.67);

        // Test rounding to 4 decimal places
        assert_eq!(round_decimal(2.718281828, 4), 2.7183);
        assert_eq!(round_decimal(9.999999999, 4), 10.0);

        // Test rounding to 0 decimal places
        assert_eq!(round_decimal(123.456789, 0), 123.0);
        assert_eq!(round_decimal(0.987654321, 0), 1.0);
    }
}

fn main() {
    println!("Hello, world!");
}
