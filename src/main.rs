use std::io;


/// The Main function
/// Tis function is endpoint of the main program
fn main() {
    let mut input = String::new();

    println!("Введите коэффициенты для первой прямой (Ax By = C)\nчерез пробел:");
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        eprintln!("Ошибка при чтении: {}", e);
        std::process::exit(1);
    });
    let (a1, b1, c1) = parse_coefficients(&input);

    println!("Введите коэффициенты для второй прямой (Dx Ey = F):\nчерез пробел:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        eprintln!("Ошибка при чтении: {}", e);
        std::process::exit(1);
    });
    let (d1, e1, f1) = parse_coefficients(&input);

    let denominator = a1 * e1 - b1 * d1;

    if denominator == 0 && (a1 != 0 || b1 != 0 || d1 != 0 || e1 != 0) {
        println!("Прямые параллельны и не пересекаются");
    } else if denominator == 0 {
        if a1 * f1 - c1 * d1 == 0 && b1 * f1 - c1 * e1 == 0 {
            println!("Прямые совпадают");
        } else {
            println!("Прямые параллельны и не пересекаются");
        }
    } else {
        let x = (c1 * e1 - b1 * f1) as f64 / denominator as f64;
        let y = (a1 * f1 - c1 * d1) as f64 / denominator as f64;
        println!("Прямые пересекаются в точке ({:.2}, {:.2})", x, y);
    }

    if a1 == 0 && b1 == 0 {
        println!("Первая прямая параллельна или совпадает с осью X");
    } else if a1 == 0 {
        println!("Первая прямая вертикальна");
    }

    if d1 == 0 && e1 == 0 {
        println!("Вторая прямая параллельна или совпадает с осью X");
    } else if d1 == 0 {
        println!("Вторая прямая вертикальна");
    }
}

fn parse_coefficients(input: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or_else(|_| {
            eprintln!("Введены неверные коэффициенты");
            std::process::exit(0);
        }))
        .collect();

    if parts.len() == 3 {
        (parts[0], parts[1], parts[2])
    } else {
        eprintln!("Введены неверные коэффициенты");
        std::process::exit(0);
    }
}
