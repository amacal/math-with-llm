fn main() {
    println!("root of x^3 + x at 5.0 = {}", find_root(|x| x * x * x + x, 5.0, 8.0, 1e-6, 100).unwrap());
}

type Function = fn(f64) -> f64;

fn find_root(f0: Function, x0: f64, x1: f64, eps: f64, n: usize) -> Option<f64> {
    let mut x0 = x0;
    let mut x1 = x1;

    if x1 == x0 {
        return None;
    }

    for _ in 0..n {
        let y0 = f0(x0);
        let y1 = f0(x1);

        let y_diff = y1 - y0;
        let x_diff = x1 - x0;

        if y_diff == 0.0 {
            return None;
        }

        let x = x1 - y1 * x_diff / y_diff;
        if (x1 - x).abs() < eps {
            return Some(x);
        }

        x0 = x1;
        x1 = x;
    }

    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn root_of_x_cube_plus_x_at_plus_1() {
        let f0 = |x: f64| x * x * x + x;

        match super::find_root(f0, 1.0, 2.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_plus_x_at_minus_1() {
        let f0 = |x: f64| x * x * x + x;
        match super::find_root(f0, -1.0, 1.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_plus_x_at_plus_100() {
        let f0 = |x: f64| x * x * x + x;
        match super::find_root(f0, 100.0, 101.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_2() {
        let f0 = |x: f64| x * x * x - 2.0;

        match super::find_root(f0, 1.0, 2.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 2.0f64.powf(1.0/3.0)).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_right() {
        let f0 = |x: f64| x * x * x - x;

        match super::find_root(f0, 2.0, 2.46, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 1.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_left() {
        let f0 = |x: f64| x * x * x - x;

        match super::find_root(f0, -2.0, -1.46, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root + 1.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_left_between() {
        let f0 = |x: f64| x * x * x - x;

        match super::find_root(f0, -0.16, -0.15, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_right_between() {
        let f0 = |x: f64| x * x * x - x;

        match super::find_root(f0, 0.15, 0.16, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_cube_minus_x_at_1_by_sqrt_5() {
        let f0 = |x: f64| x * x * x - x;

        match super::find_root(f0, 1.0 / 5.0f64.sqrt(), 1.1 / 5.0f64.sqrt(), 1e-6, 100) {
            None => assert!(true),
            Some(root) => assert!((root - 0.0).abs() < 1e-5),
        };
    }


    #[test]
    fn root_of_x_minus_1_square_at_1_0001() {
        let f0 = |x: f64| (x - 1.0) * (x - 1.0);

        match super::find_root(f0, 1.0001, 1.0005, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 1.0).abs() < 1e-5),
        };
    }

    #[test]
    fn root_of_x_minus_1_square_at_10001() {
        let f0 = |x: f64| (x - 1.0) * (x - 1.0);

        match super::find_root(f0, 10001.0, 10005.0, 1e-6, 100) {
            None => assert!(false),
            Some(root) => assert!((root - 1.0).abs() < 1e-5),
        };
    }
}
