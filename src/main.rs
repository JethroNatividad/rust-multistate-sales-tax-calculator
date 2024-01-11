use std::io;
use std::io::Write;
// Tax calculator, State: wisconsin | illinois (8% tax), if wisconsin - county:  Eau Claire (+0.005) | Dunn (0.004) , other states: no tax.
// Inputs: order_amount, state, county(only if state: wisconsin)
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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

// Inputs: order_amount, state, county(only if state: wisconsin)
// process: calculate tax
// output: The tax is: {}.\nThe total is {}.

struct County {
    names: Vec<String>,
    tax: f64,
}
struct State {
    names: Vec<String>,
    tax: f64,
    counties: Vec<County>,
}

fn main() {
    // write states
    let states: Vec<State> = vec![
        State {
            names: vec!["wisconsin".to_string(), "wi".to_string()],
            tax: 5.0,
            counties: vec![
                County {
                    names: vec!["eau claire".to_string(), "eau claire county".to_string()],
                    tax: 0.5,
                },
                County {
                    names: vec!["dunn".to_string(), "dunn county".to_string()],
                    tax: 0.4,
                },
            ],
        },
        State {
            names: vec!["illinois".to_string(), "il".to_string()],
            tax: 8.0,
            counties: vec![],
        },
    ];
    // prompt order_amount "What is the order amount? "
    // prompt state "What state do you live in? "
    // calculate the tax
    // if state has
    // counties, prompt "What county do you live in? "
    // add the extra charge to the tax.
    // display the tax and total.
    println!("Hello world!")
}
